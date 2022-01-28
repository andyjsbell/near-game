# `near-sdk-rs` Starter Kit

This is a good project to use as a starting point for your AssemblyScript project.

## Samples

This repository includes a complete project structure for AssemblyScript contracts targeting the NEAR platform.

The example here is very basic.  It's a simple contract demonstrating the following concepts:
- a single contract
- the difference between `view` vs. `change` methods

There are 1 Rust contracts in this project, each in their own folder:

- **status message** in the `src` folder

### Simple

We say that an rust contract is written in the "simple style" when the `lib.rs` file (the contract entry point) includes a series of exported functions.

In this case, all exported functions become public contract methods.




## Usage

### Getting started

(see below for video recordings of each of the following steps)

INSTALL `NEAR CLI` first like this: `npm i -g near-cli`
INSTALL RUST toolchain
Add the wasm target using `rustup target add wasm32-unknown-unknown`

1. clone this repo to a local folder
2. run `./scripts/1.dev-deploy.sh`
3. run `./scripts/2.use-contract.sh`
4. run `./scripts/3.cleanup.sh`

### Videos

**`1.dev-deploy.sh`**

This video shows the build and deployment of the contract.

[![asciicast](https://asciinema.org/a/409575.svg)](https://asciinema.org/a/409575)

**`2.use-contract.sh`**

This video shows contract methods being called.  You should run the script twice to see the effect it has on contract state.

[![asciicast](https://asciinema.org/a/409577.svg)](https://asciinema.org/a/409577)

**`3.cleanup.sh`**

This video shows the cleanup script running.  Make sure you add the `BENEFICIARY` environment variable. The script will remind you if you forget.

```sh
export BENEFICIARY=<your-account-here>   # this account receives contract account balance
```

[![asciicast](https://asciinema.org/a/409580.svg)](https://asciinema.org/a/409580)

### Other documentation

- See `./scripts/README.md` for documentation about the scripts
- Watch this video where Willem Wyndham walks us through refactoring a simple example of a NEAR smart contract written in AssemblyScript

  https://youtu.be/QP7aveSqRPo

