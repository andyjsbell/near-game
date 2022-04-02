use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::Serialize;
use near_sdk::PanicOnDefault;
use near_sdk::{env, log, near_bindgen, require, AccountId, BorshStorageKey};
use std::collections::VecDeque;

/// A trait describing a simple turn based game engine
trait Engine {
    type State;
    type Player;
    type Turn;
    /// Number of players for this game
    fn number_of_players() -> u8;
    /// Is the game finished, if so return the winner
    fn is_finished(&self) -> Option<Self::Player>;
    /// Play a turn in the game
    fn play_turn(&mut self, turn: Self::Turn, player: Self::Player) -> Option<Self::State>;
}

/// Connect four, https://en.wikipedia.org/wiki/Connect_Four
mod connect_four {
    use super::*;
    /// An identitifier for a player
    type Player = u8;
    /// The board for the game 6x7
    type Board = [[Player; 6]; 7];
    /// An identifier for the column
    type Column = u8;
    /// The number of players
    const NUMBER_OF_PLAYERS: u8 = 2;

    /// The game with a board
    #[near_bindgen]
    #[derive(Default, Serialize, BorshDeserialize, BorshSerialize)]
    #[serde(crate = "near_sdk::serde")]
    pub struct Game {
        cells: Board,
    }

    impl Engine for Game {
        type State = Board;
        type Player = Player;
        type Turn = Column;

        fn number_of_players() -> u8 {
            NUMBER_OF_PLAYERS
        }

        fn is_finished(&self) -> Option<Self::Player> {
            let is_player_finished = |player: u8| {
                for y in 0..self.cells[0].len() {
                    for x in 0..self.cells.len() - 3 {
                        if self.cells[x][y] == player
                            && self.cells[x + 1][y] == player
                            && self.cells[x + 2][y] == player
                            && self.cells[x + 3][y] == player
                        {
                            return Some(self.cells);
                        }
                    }
                }

                for y in 0..self.cells[0].len() - 3 {
                    for board_x in self.cells {
                        if board_x[y] == player
                            && board_x[y + 1] == player
                            && board_x[y + 2] == player
                            && board_x[y + 3] == player
                        {
                            return Some(self.cells);
                        }
                    }
                }

                for y in 0..self.cells[0].len() - 3 {
                    for x in 3..self.cells.len() {
                        if self.cells[x][y] == player
                            && self.cells[x - 1][y + 1] == player
                            && self.cells[x - 2][y + 2] == player
                            && self.cells[x - 3][y + 3] == player
                        {
                            return Some(self.cells);
                        }
                    }
                }

                for y in 3..self.cells[0].len() {
                    for x in 3..self.cells.len() {
                        if self.cells[x][y] == player
                            && self.cells[x - 1][y - 1] == player
                            && self.cells[x - 2][y - 2] == player
                            && self.cells[x - 3][y - 3] == player
                        {
                            return Some(self.cells);
                        }
                    }
                }
                None
            };

            for player in 1..Self::number_of_players() + 1 {
                match is_player_finished(player) {
                    Some(_) => return Some(player),
                    None => continue,
                }
            }

            None
        }

        fn play_turn(&mut self, turn: Self::Turn, player: Self::Player) -> Option<Self::State> {
            if self.cells[turn as usize][0] > 0 {
                return None;
            }

            let board_rows: usize = self.cells[0].len();
            for y in 0..board_rows {
                let y_pos = board_rows - y - 1;
                if self.cells[turn as usize][y_pos] > 0 {
                    continue;
                }
                self.cells[turn as usize][y_pos] = player;
                break;
            }

            Some(self.cells)
        }
    }
}

type Identifier = u32;

#[near_bindgen]
#[derive(Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Game {
    /// An identifier for this game
    identifier: Identifier,
    /// Players in the game
    players: Vec<AccountId>,
    /// Next player
    next_player: u8,
    /// The winner of the game
    winner: Option<AccountId>,
    /// The game logic
    engine: connect_four::Game,
}

impl Game {
    pub fn increment_turn(&mut self) {
        if self.next_player == connect_four::Game::number_of_players() {
            self.next_player = 1;
        } else {
            self.next_player += 1;
        }
    }

    pub fn set_winner(&mut self, winner: AccountId) {
        self.winner = Some(winner);
    }

    pub fn winner(&self) -> Option<AccountId> {
        self.winner.clone()
    }
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    /// A counter for the game identifier
    game_counter: Identifier,
    /// Map of games by identifier
    games: LookupMap<Identifier, Game>,
    /// Queue of players
    player_queue: VecDeque<AccountId>,
}

/// Storage keys
#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    /// Key for Games storage collection
    Games,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        require!(!env::state_exists(), "Already initialized");
        Contract {
            game_counter: 0,
            games: LookupMap::new(StorageKey::Games),
            player_queue: Default::default(),
        }
    }

    /// Queue the signer for the next game.  If we have players in the queue
    /// then the player is queued and a game is started, else they signer is queued.
    /// The signer can poll this function or wait on an event to call to get the game
    /// they have been scheduled in.
    pub fn queue(&mut self) -> Option<Game> {
        let required_number_of_players =
            connect_four::Game::number_of_players().saturating_sub(1) as usize;

        return if self.player_queue.len() as usize >= required_number_of_players {
            // Take the players at the head of the vector
            let mut players: Vec<_> = self
                .player_queue
                .drain(0..required_number_of_players)
                .collect();

            // Add the signer as player
            players.push(env::signer_account_id());

            // If we ever get there, overflow
            self.game_counter += 1;

            let game = Game {
                players: players.clone(),
                identifier: self.game_counter,
                next_player: 1,
                winner: None,
                engine: Default::default(),
            };

            log!("Game Created with players: {:?}", players);

            self.games.insert(&game.identifier, &game);
            Some(game)
        } else {
            // Queue the signer if we aren't yet in the call
            if self
                .player_queue
                .iter()
                .find(|player| env::signer_account_id() == **player)
                .is_none()
            {
                self.player_queue.push_back(env::signer_account_id());
            }
            None
        };
    }

    /// Find player index in the game
    fn find_player(account_id: AccountId, game: &Game) -> Option<usize> {
        game.players
            .iter()
            .enumerate()
            .find(|(_, player)| **player == account_id)
            .map(|(idx, _)| idx)
    }

    /// Increment the game turn
    fn increment_game_turn(&mut self, game: &mut Game) {
        game.increment_turn();
        self.games.insert(&game.identifier, game);
    }

    /// Play a turn in the game, fails if it isn't our turn
    /// Column's are indexed from 0
    pub fn play(&mut self, identifier: Identifier, column: u8) -> Option<AccountId> {
        let mut game = self.games.get(&identifier).expect("game does not exist");
        require!(game.winner == None, "the game has completed");

        let players_turn =
            Self::find_player(env::signer_account_id(), &game).expect("player to be in game") + 1;

        require!(
            players_turn == game.next_player as usize,
            "not player's turn"
        );

        game.engine
            .play_turn(column, players_turn as u8)
            .expect("invalid turn played");

        self.increment_game_turn(&mut game);
        self.is_finished(identifier)
    }

    /// Is the game finished, if so the winner will be returned
    pub fn is_finished(&mut self, identifier: Identifier) -> Option<AccountId> {
        let mut game = self.games.get(&identifier).expect("game does not exist");

        return match game.winner() {
            Some(_) => game.winner(),
            None => {
                return match game.engine.is_finished() {
                    Some(player) => {
                        game.set_winner(game.players[(player - 1) as usize].clone());
                        self.games.insert(&game.identifier, &game);
                        Some(game.players[(player - 1) as usize].clone())
                    }
                    None => None,
                };
            }
        };
    }

    /// Get a game by its identifier
    pub fn get_game(&self, identifier: Identifier) -> Option<Game> {
        self.games.get(&identifier)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, get_logs, VMContextBuilder};
    use near_sdk::testing_env;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn should_queue_and_create_game() {
        let mut context = get_context(accounts(0));
        testing_env!(context.signer_account_id(accounts(1)).build());
        let mut contract = Contract::new();
        assert!(
            contract.queue().is_none(),
            "as we are the first we should have no game to play"
        );
        testing_env!(context.signer_account_id(accounts(2)).build());
        assert!(
            contract.queue().is_some(),
            "we would now have a game as we have two players"
        );
        assert_eq!(
            get_logs(),
            vec![format!(
                "Game Created with players: {:?}",
                [accounts(1), accounts(2)]
            )],
            "we are expecting a log with the players in the new game"
        );
        assert!(
            contract.queue().is_none(),
            "as the queue was cleared we are the first again and we should have no game to play"
        );
    }

    #[test]
    #[should_panic(expected = "not player's turn")]
    fn should_not_be_able_to_play_turn_twice() {
        let mut context = get_context(accounts(0));
        testing_env!(context.signer_account_id(accounts(1)).build());
        let mut contract = Contract::new();
        assert!(contract.queue().is_none(), "no game yet");
        testing_env!(context.signer_account_id(accounts(2)).build());
        let Game { identifier, .. } = contract.queue().expect("game created");
        // Player 1
        testing_env!(context.signer_account_id(accounts(1)).build());
        contract.play(identifier, 1);
        // Player 2
        testing_env!(context.signer_account_id(accounts(2)).build());
        contract.play(identifier, 1);
        // Player 1
        testing_env!(context.signer_account_id(accounts(1)).build());
        contract.play(identifier, 1);
        // Try to play again
        contract.play(identifier, 1);
    }

    #[test]
    #[should_panic(expected = "invalid turn played")]
    fn should_prevent_invalid_plays() {
        let mut context = get_context(accounts(0));
        testing_env!(context.signer_account_id(accounts(1)).build());
        let mut contract = Contract::new();
        assert!(contract.queue().is_none(), "no game yet");
        testing_env!(context.signer_account_id(accounts(2)).build());
        let Game { identifier, .. } = contract.queue().expect("game created");
        // Player 1
        testing_env!(context.signer_account_id(accounts(1)).build());
        contract.play(identifier, 1);

        // Player 2
        testing_env!(context.signer_account_id(accounts(2)).build());
        contract.play(identifier, 1);

        // Player 1
        testing_env!(context.signer_account_id(accounts(1)).build());
        contract.play(identifier, 1);

        // Player 2
        testing_env!(context.signer_account_id(accounts(2)).build());
        contract.play(identifier, 1);

        // Player 1
        testing_env!(context.signer_account_id(accounts(1)).build());
        contract.play(identifier, 1);

        // Player 2
        testing_env!(context.signer_account_id(accounts(2)).build());
        contract.play(identifier, 1);

        // Player 1
        // This would be a filled column 1
        testing_env!(context.signer_account_id(accounts(1)).build());
        contract.play(identifier, 1);
    }

    #[test]
    #[should_panic(expected = "the game has completed")]
    fn should_win_game_and_prevent_further_plays() {
        let mut context = get_context(accounts(0));
        testing_env!(context.signer_account_id(accounts(1)).build());
        let mut contract = Contract::new();
        contract.queue();
        testing_env!(context.signer_account_id(accounts(2)).build());
        let Game { identifier, .. } = contract.queue().expect("game created");

        // Player 1
        // One..
        testing_env!(context.signer_account_id(accounts(1)).build());
        contract.play(identifier, 1);

        // Player 2
        testing_env!(context.signer_account_id(accounts(2)).build());
        contract.play(identifier, 2);

        // Player 1
        // Two..
        testing_env!(context.signer_account_id(accounts(1)).build());
        contract.play(identifier, 1);

        // Player 2
        testing_env!(context.signer_account_id(accounts(2)).build());
        contract.play(identifier, 3);

        // Player 1
        // Three..
        testing_env!(context.signer_account_id(accounts(1)).build());
        contract.play(identifier, 1);

        // Player 2
        testing_env!(context.signer_account_id(accounts(2)).build());
        contract.play(identifier, 4);

        // Player 1
        // Four..win!
        testing_env!(context.signer_account_id(accounts(1)).build());
        assert_eq!(
            contract.play(identifier, 1),
            Some(accounts(1)),
            "player one should have won"
        );

        // Player 2 sends their turn
        testing_env!(context.signer_account_id(accounts(2)).build());
        contract.play(identifier, 1);
    }
}
