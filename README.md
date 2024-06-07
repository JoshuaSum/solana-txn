# Solana Block Listener

This project is a Rust application that listens for new blocks on the Solana blockchain, fetches block details, and prints transaction information to the console. It connects to the Solana devnet RPC endpoint and periodically polls for new blocks, printing relevant transaction details.

## Features

- Connects to the Solana devnet RPC endpoint.
- Periodically polls for new blocks within a specified slot range.
- Fetches detailed transaction information for each block.
- Prints transaction signatures, recent blockhashes, and instructions.

## Requirements

- Rust programming language (version 1.50.0 or later)

## Dependencies

In the `Cargo.toml` file, you'll find the following dependencies:

```toml
[dependencies]
solana-client = "1.18.12"
solana-sdk = "1.18.12"
solana-transaction-status = "1.18.12"
tokio = "1.37.0"
serde_json = "1.0"
```

- `solana-client`: The Solana RPC client library providing the necessary functions to interact with Solana RPC endpoints.
- `solana-sdk`: The Solana SDK library that provides the necessary functions to interact with the HTTP or Websocket client.
- `solana-transaction-status`: The library including essential types and utilities that we will use for message encoding/decoding and to handle instructions in each transaction.
- `serde_json`: JSON serialization/deserialization library
- `tokio`: The asynchronous runtime that is used in this case to handle WebSocket communication and timers.

## Installation

1. Ensure you have Rust installed. If not, you can get it from [rust-lang.org](https://www.rust-lang.org/).

2. Clone this repository:

    ```sh
    git clone git@github.com:JoshuaSum/solana-txn.git
    cd solana-block-listener
    ```

3. Build the project:

    ```sh
    cargo build --release
    ```

## Usage

Start the block listener by running the following command

```sh
cargo run --release
```

## Websocket Subscriber
The project also includes an example file websocket.rs that demonstrates how to subscribe to incoming blocks using the PubsubClient from the solana_client. Note that this requires the Solana node to be running with the flag --rpc-pubsub-enable-block-subscription.