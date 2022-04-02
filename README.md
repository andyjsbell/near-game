# `near-game` A simple near smart contract game

This repository includes one smart contract which allows two players to play a game of connect four, 
https://en.wikipedia.org/wiki/Connect_Four.
It also provides a script to play a game for demonstration purposes.

NB. At present it doesn't include any dispute mechanism as a game can be blocked if a player doesn't play their turn for whatever reason.
## Usage

### Getting started

INSTALL `NEAR CLI` first like this: `npm i -g near-cli`
INSTALL RUST toolchain
Add the wasm target using `rustup target add wasm32-unknown-unknown`

1. clone this repo to a local folder
2. run `./build.sh`
3. run `./scripts/run-game.sh`
