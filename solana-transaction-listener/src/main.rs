use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use std::{thread, time::Duration};

fn main() {
    // Connect to the Solana devnet RPC endpoint
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    // Start from the most recent slot
    let mut start_slot = client.get_slot().unwrap_or(0);
    let block_range = 10;

    loop {
        let end_slot = start_slot + block_range;
        match client.get_blocks(start_slot, Some(end_slot)) {
            Ok(blocks) => {
                if blocks.is_empty() {
                    println!("No new blocks found from slot {} to {}", start_slot, end_slot);
                } else {
                    println!("Blocks from slot {} to {}: {:?}", start_slot, end_slot, blocks);
                }
            }
            Err(err) => {
                eprintln!("Failed to get blocks from slot {} to {}: {:?}", start_slot, end_slot, err);
            }
        }

        // Update the start slot to the end slot for the next iteration
        start_slot = end_slot;

        // Small delay to prevent overwhelming the RPC server
        thread::sleep(Duration::from_secs(5));
    }
}
