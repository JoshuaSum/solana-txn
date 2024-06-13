use solana_gossip::crds::Cursor;
use solana_gossip::gossip_service::make_gossip_node;
use solana_streamer::socket::SocketAddrSpace;
use solana_sdk::signature::Keypair;
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::atomic::AtomicBool;
use std::thread::sleep;
use std::time::Duration;
use std::sync::Arc;

fn main() {
    // Create a gossip node keypair
    let node_keypair = Keypair::new();

    // Create cluster info with a gossip entry point to an existing Solana cluster
    let entrypoint: SocketAddr = {
        let mut addrs = "api.devnet.solana.com:8001".to_socket_addrs().unwrap();
        addrs.next().unwrap()
    };
    let socket_addr_space: SocketAddrSpace = SocketAddrSpace::new(false);

    // Start the gossip service
    let (gossip_service, _listener, cluster_info) = make_gossip_node(
        node_keypair,
        Some(&entrypoint),
        Arc::new(AtomicBool::new(false)),
        None,
        0,
        true,
        socket_addr_space,
    );

    // Setup cursor for CRDS (Cluster Replicated Data Store)
    let mut cursor = Cursor::default();

    // Loop to fetch and print gossip entries
    loop {
        let transactions = cluster_info.get_votes(& mut cursor);

        for txn in transactions {
            println!("  Transaction detected:");
            for signature in txn.signatures{
                println!("    Signature: {}", signature);
            };
            println!("    Message Header:");
            println!("        Required Signatures: {}", txn.message.header.num_required_signatures);
            println!("        Read-only Signed Accounts: {}", txn.message.header.num_readonly_signed_accounts);
            println!("        Read-only Unsigned Accounts: {}", txn.message.header.num_readonly_unsigned_accounts);
            println!("    Account Keys:");
            for pub_key in txn.message.account_keys{
                println!("        Key: {}", pub_key.to_string());
            }
            println!("    Recent Blockhash: {}", txn.message.recent_blockhash.to_string());
            println!("    Instructions:");
            for instruction in txn.message.instructions{
                println!("        Program ID: {}", instruction.program_id_index);
                println!("        Accounts:");
                for account in instruction.accounts{
                    println!("        Account: {}", account);
                }
                print!("        Program input data:");
                for data in instruction.data{
                    print!(" {}", data);
                }
            }
            println!();
        }

        // Small delay to prevent overwhelming the system
        sleep(Duration::from_secs(5));
    }

    // Shutdown gossip service (though it won't reach here in this infinite loop)
    drop(gossip_service);
}
