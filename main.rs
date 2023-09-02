use p2pledger::peer::{ PeerManager};
use p2pledger::models::{Blockchain, Transaction};
#[tokio::main]
async fn main() {

    // Create a new blockchain instance
    let mut blockchain = Blockchain::new();

    // Add a dummy transaction for testing
    blockchain.add_block(vec![Transaction {
        sender: "Alice".to_string(),
        recipient: "Bob".to_string(),
        amount: 50,
    }]);

    // Create the peer manager
    let peer_manager = PeerManager::new();

    // Start P2P server
    p2pledger::network::start_server(&peer_manager, blockchain).await;
}
