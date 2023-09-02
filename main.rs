use p2pledger::{Peer};


fn main() {
    // Define own peer address and create a new Peer instance
    let peer_address = String::from("127.0.0.1:8085");
    let peer = Peer::new(peer_address.clone(), blockchain);
    //sds

}
