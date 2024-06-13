use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcBlockConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::{EncodedTransaction, UiInstruction, UiMessage, UiParsedInstruction};
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
                    for block_slot in blocks {
                        match client.get_block_with_config(
                            block_slot,
                            RpcBlockConfig {
                                encoding: Some(solana_transaction_status::UiTransactionEncoding::Json),
                                transaction_details: Some(solana_transaction_status::TransactionDetails::Full),
                                rewards: Some(false),
                                commitment: Some(CommitmentConfig::confirmed()),
                                max_supported_transaction_version: None,
                            },
                        ) {
                            Ok(block) => {
                                println!("New block received: Slot {}", block_slot);
                                for txn in &block.transactions {
                                    for transaction_with_meta in txn {
                                        match &transaction_with_meta.transaction {
                                            EncodedTransaction::Json(transaction_json) => {
                                                let signatures = &transaction_json.signatures;
                                                let message = &transaction_json.message;
    
                                                let signature = signatures[0].to_string();
    
                                                match message {
                                                    UiMessage::Parsed(parsed_message) => {
                                                        println!("  Transaction detected:");
                                                        println!("    Signature: {}", signature);
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
                                eprintln!("Failed to get block for slot {}: {:?}", block_slot, err);
                            }
                        }
                    }
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
