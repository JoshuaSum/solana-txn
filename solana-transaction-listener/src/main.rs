use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcBlockConfig;
use solana_gossip::crds::Cursor;
use solana_gossip::gossip_service::make_gossip_node;
use solana_streamer::socket::SocketAddrSpace;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Keypair;
use solana_transaction_status::{EncodedTransaction, UiInstruction, UiMessage, UiParsedInstruction};
use std::net::{SocketAddr, UdpSocket};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Connect to the Solana devnet RPC endpoint
    let rpc_url = "https://api.devnet.solana.com";
    let rpc_client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    // Create a gossip node keypair
    let node_keypair = Keypair::new();

    // Create cluster info with a gossip entry point to an existing Solana cluster
    let entrypoint: SocketAddr = "api.devnet.solana.com:8001".parse().unwrap();
    let socket_addr_space: SocketAddrSpace = SocketAddrSpace::new(false);

    // Create the node and bind to the local UDP socket
    let gossip_socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind to UDP socket");

    // Start the gossip service
    let (gossip_service, listener, cluster_info) = make_gossip_node(
        node_keypair,
        &entrypoint,
        None,
        &gossip_socket,
        None,
        true,
        socket_addr_space,
    );

    // Setup cursor for CRDS (Cluster Replicated Data Store)
    let mut cursor = Cursor::default();

    // Loop to fetch and print gossip entries
    loop {
        let epoch_slots_data = cluster_info.get_epoch_slots(&mut cursor);

        for epoch_slots in epoch_slots_data {
            let slot_vec = epoch_slots.to_slots(1);
            for slot in slot_vec {
                println!("CRDS Entry Slot: {:?}", slot);
                
                // Fetch the block details using the slot number
                match rpc_client.get_block_with_config(
                    slot,
                    RpcBlockConfig {
                        encoding: Some(solana_transaction_status::UiTransactionEncoding::Json),
                        transaction_details: Some(solana_transaction_status::TransactionDetails::Full),
                        rewards: Some(false),
                        commitment: Some(CommitmentConfig::confirmed()),
                        max_supported_transaction_version: None,
                    },
                ) {
                    Ok(block) => {
                        println!("New block received: Slot {}", slot);

                        for txn in &block.transactions {
                            for transaction_with_meta in txn{
                                match &transaction_with_meta.transaction {
                                    EncodedTransaction::Json(transaction_json) => {
                                        let signatures = &transaction_json.signatures;
                                        let message = &transaction_json.message;

                                        let signature = signatures[0].to_string();

                                        println!("  Transaction detected:");
                                        println!("    Signature: {}", signature);

                                        match message {
                                            UiMessage::Parsed(parsed_message) => {
                                                println!("    Recent Blockhash: {}", parsed_message.recent_blockhash);
                                                println!("    Instructions:");

                                                for instruction in &parsed_message.instructions {
                                                    match instruction {
                                                        UiInstruction::Parsed(parsed_instruction) => {
                                                            match parsed_instruction {
                                                                UiParsedInstruction::Parsed(instruction_parsed) => {
                                                                    println!("      Program ID Index: {}", instruction_parsed.program_id);
                                                                    println!("      Data: {:?}", instruction_parsed.program);
                                                                }
                                                                UiParsedInstruction::PartiallyDecoded(instruction_partial) => {
                                                                    println!("      Program ID Index: {}", instruction_partial.program_id);
                                                                    println!("      Data: {:?}", instruction_partial.data);
                                                                }
                                                            }
                                                        }
                                                        UiInstruction::Compiled(compiled_instruction) => {
                                                            println!("      Program ID Index: {}", compiled_instruction.program_id_index);
                                                            println!("      Data: {:?}", compiled_instruction.data);
                                                            println!("      Accounts: {:?}", compiled_instruction.accounts);
                                                        }
                                                    }
                                                }
                                            }
                                            UiMessage::Raw(raw_message) => {
                                                println!("  Raw message detected:");
                                                println!("    Recent Blockhash: {}", raw_message.recent_blockhash);
                                                println!("    Instructions:");

                                                for instruction in &raw_message.instructions {
                                                    println!("      Program ID Index: {}", instruction.program_id_index);
                                                    println!("      Data: {:?}", instruction.data);
                                                    println!("      Accounts: {:?}", instruction.accounts);
                                                }
                                            }
                                        }

                                        println!();
                                    }
                                    EncodedTransaction::Binary(_, _) => {
                                        println!("  Binary transaction detected (skipping)");
                                    }
                                    EncodedTransaction::LegacyBinary(_) => {
                                        println!("  Legacy binary transaction detected (skipping)");
                                    }
                                    EncodedTransaction::Accounts(_) => {
                                        println!("  Accounts detected (skipping)");
                                    }
                                }
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Failed to get block for slot {}: {:?}", slot, err);
                    }
                }
            }
        }

        // Small delay to prevent overwhelming the system
        sleep(Duration::from_secs(5));
    }

    // Shutdown gossip service (though it won't reach here in this infinite loop)
    drop(gossip_service);
}
