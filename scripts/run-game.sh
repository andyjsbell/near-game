#!/usr/bin/env bash

rm -f ./neardev/dev-account
rm -f ./neardev/dev-account.env

#build for wasm target
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
# !! Change the beneficiary !!
BENEFICIARY=barkingmad.testnet

echo ---------------------------------------------------------
echo "Deploying developer contract"
echo ---------------------------------------------------------
echo
near dev-deploy ./target/wasm32-unknown-unknown/release/near_game.wasm --initialBalance "10"
CONTRACT=`cat ./neardev/dev-account`
echo
echo ---------------------------------------------------------
echo "Developer account: " $CONTRACT  
echo ---------------------------------------------------------
echo
echo ---------------------------------------------------------
echo "Create player 1"
echo ---------------------------------------------------------
echo 
near create-account player1.$CONTRACT --masterAccount $CONTRACT --initialBalance "1"
echo 
echo ---------------------------------------------------------
echo "Create player 2"
echo ---------------------------------------------------------
echo
near create-account player2.$CONTRACT --masterAccount $CONTRACT --initialBalance "1"
echo
echo ---------------------------------------------------------
echo "Initialising Contract"
echo ---------------------------------------------------------
echo 
near call $CONTRACT new --accountId $CONTRACT
echo
echo ---------------------------------------------------------
echo "Queue player 1"
echo ---------------------------------------------------------
echo
near call $CONTRACT queue --accountId player1.$CONTRACT
echo
echo ---------------------------------------------------------
echo "Queue player 2"
echo ---------------------------------------------------------
echo
near call $CONTRACT queue --accountId player2.$CONTRACT
echo
echo ---------------------------------------------------------
echo "Game created!"
echo "Let us play, stack in two columns, player 1 would win"
echo "As this is the first game its identifer will be 1"
echo ---------------------------------------------------------
echo "Player 1 column 1"
echo ---------------------------------------------------------
echo
near call $CONTRACT play --args '{"identifier": 1, "column": 0}' --accountId player1.$CONTRACT 
echo
echo ---------------------------------------------------------
echo "Player 2 column 2"
echo ---------------------------------------------------------
echo 
near call $CONTRACT play --args '{"identifier": 1, "column": 1}' --accountId player2.$CONTRACT 
echo
echo ---------------------------------------------------------
echo "Player 1 column 1"
echo ---------------------------------------------------------
echo
near call $CONTRACT play --args '{"identifier": 1, "column": 0}' --accountId player1.$CONTRACT 
echo
echo ---------------------------------------------------------
echo "Player 2 column 2"
echo ---------------------------------------------------------
echo
near call $CONTRACT play --args '{"identifier": 1, "column": 1}' --accountId player2.$CONTRACT 
echo
echo ---------------------------------------------------------
echo "Player 1 column 1"
echo ---------------------------------------------------------
echo
near call $CONTRACT play --args '{"identifier": 1, "column": 0}' --accountId player1.$CONTRACT 
echo
echo ---------------------------------------------------------
echo "Player 2 column 2"
echo ---------------------------------------------------------
echo
near call $CONTRACT play --args '{"identifier": 1, "column": 1}' --accountId player2.$CONTRACT 
echo
echo ---------------------------------------------------------
echo "Player 1 column 1, player 1 would have 4 in a row!"
echo ---------------------------------------------------------
echo
near call $CONTRACT play --args '{"identifier": 1, "column": 0}' --accountId player1.$CONTRACT 
echo
echo ---------------------------------------------------------
echo "Let's get the state of the game"
echo ---------------------------------------------------------
echo
near view $CONTRACT get_game --args '{"identifier": 1}'
echo
echo ---------------------------------------------------------
echo "Removing tests accounts with beneficiary " $BENEFICIARY
echo ---------------------------------------------------------
echo 
near delete player1.$CONTRACT $BENEFICIARY
near delete player2.$CONTRACT $BENEFICIARY
near delete $CONTRACT $BENEFICIARY
echo "All done!"
exit 0
