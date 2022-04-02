# `near-game` A simple near smart contract game

This repository includes one smart contract which allows two players to play a game of connect four, 
https://en.wikipedia.org/wiki/Connect_Four.  

It explores the concept of running a *basic* turn based game on the Near blockchain.  Gaming is an interesting branch of Web3 and Near provides transaction speeds that can provide an expected user experience in the gaming world.

The project also provides a script to play a game for demonstration purposes.

## Things to do
- At present the contract doesn't include any dispute mechanism as a game can be blocked if a player doesn't play their turn for whatever reason.
- Oh, and the board is stored where the rows and columns are inverted to make it easier in memory.  This would need to be visualised by inverting them back at some point. 

## Usage

### Getting started

- INSTALL `NEAR CLI` first like this: `npm i -g near-cli`  
- INSTALL RUST toolchain  
- Add the wasm target using `rustup target add wasm32-unknown-unknown`

### Build and running
- Clone this repo to a local folder
- Run `./build.sh`
- Run `./scripts/run-game.sh`
