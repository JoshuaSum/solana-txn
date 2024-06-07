use solana_client::pubsub_client::PubsubClient;
use solana_client::rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::UiTransactionEncoding;
use tokio::time::{sleep, Duration};

const SOLANA_RPC_URL_WS: &str = "wss://api.devnet.solana.com";

#[tokio::main]
async fn main() {
    println!("Solana Subscription Example!");

    // Define the configuration for the subscription
    let config = RpcBlockSubscribeConfig {
        commitment: Some(CommitmentConfig::confirmed()),
        encoding: Some(UiTransactionEncoding::Base64),
        transaction_details: Some(solana_transaction_status::TransactionDetails::Full),
        show_rewards: Some(false),
        max_supported_transaction_version: Some(0),
    };

    // Create the subscription with the defined configuration
    let subscription_result = PubsubClient::block_subscribe(&SOLANA_RPC_URL_WS, RpcBlockSubscribeFilter::All, Some(config));

    // Handle the result of the subscription attempt
    match subscription_result {
        Ok((_tx_confirmed_block, rx_confirmed_block)) => {
            // Loop through the subscription responses and print the block slot
            loop {
                match rx_confirmed_block.recv() {
                    Ok(response) => {
                        println!("{}", response.value.slot.to_string());
                    }
                    Err(e) => {
                        println!("Block Subscription Error: {:?}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to subscribe to block: {:?}", e);
        }
    }

    // Add a sleep to prevent the program from exiting immediately (if needed)
    sleep(Duration::from_secs(1)).await;
}
