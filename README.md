# Substrate "Build a Proof-of-Existence dApp" Tutorial Solution

This repository hosts a completed working version of the [Substrate](https://www.substrate.io/)
["Build a Proof-of-Existence dApp" tutorial](https://substrate.dev/docs/en/tutorials/build-a-dapp/), including a
front-end.

## Upstream

This project is built on the [Substrate Developer Hub](https://substrate.dev/)
[Node Template](https://github.com/substrate-developer-hub/substrate-node-template) &
[Front-End Template](https://github.com/substrate-developer-hub/substrate-front-end-template) and requires the same set
of dependencies (Rust, Node.js, Yarn, etc).

## Build & Run the Node

To build and run a development node for demonstration purposes, execute the following commands from the project's root
directory:

```shell
# Build the node
cargo build --release

# Purge existing chain state
./target/release/poe-node purge-chain --dev

# Start a development node
./target/release/poe-node --dev
```
## Build & Run the Front-End

To build and run the custom front-end for this project, execute the following commands from the `front-end` directory:

```shell
# Install dependencies
yarn

# Start the front-end
yarn start
```

After a few seconds, your browser should automatically open to http://localhost:8000/.

## Interact

Once the node and front-end are running, you can
[follow the steps from the official tutorial to interact with your proof-of-existence chain](https://substrate.dev/docs/en/tutorials/build-a-dapp/front-end#submit-a-proof)
using the custom front-end.
