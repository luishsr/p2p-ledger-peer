use p2pledger::{Peer};


fn main() {
    // Define own peer address and create a new Peer instance
    let peer_address = String::from("127.0.0.1:8083");
    let peer = Peer::new(peer_address.clone());
    
    // Start listening for incoming connections
    tokio::spawn(async move {
        peer.listen().await;
    });

    //peer.connect(PEER_ADDRESS).await;

}
