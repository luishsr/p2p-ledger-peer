use p2pledger::peer::{ Peer};
use p2pledger::models::{Blockchain, Transaction};
use std::io;
#[tokio::main]
async fn main() {

    println!("P2P Blockchain - V1.0 - by Luis Soares");
    // Create a new blockchain instance
    let blockchain = Blockchain::new();

    // Define own peer address and create a new Peer instance
    let peer_address = String::from("127.0.0.1:8081");
    let peer = Peer::new(peer_address.clone(), blockchain);

    // Start listening for incoming connections
    tokio::spawn(async move {
        peer.listen().await;
    });

    // Main application loop can be added here
    loop {
        // Application logic here
        println!("Enter a command (connect, send, or exit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let input = input.trim().to_lowercase();

        match input.as_str() {
            "connect" => {
                println!("Enter the Peer address to connect:");
                let mut connection_string = String::new();
                io::stdin()
                    .read_line(&mut connection_string)
                    .expect("Failed to read input");
                // Connect
                connect(connection_string.trim(), Peer::new(peer_address.clone(), Blockchain::new())).await;
            }
            "send" => {
                println!("Enter the Peer address to send the transaction:");
                let mut peer_to_send = String::new();
                io::stdin()
                    .read_line(&mut peer_to_send)
                    .expect("Failed to read input");
                // Send transation
                send(peer_to_send.trim(), Peer::new(peer_address.clone(), Blockchain::new())).await;
            },
            "exit" => {
                println!("Exiting the program.");
                break;
            }
            _ => println!("Invalid command. Try again."),
        }
    }
}

async fn connect(connection_string: &str, peer: Peer) {
    println!("Connect command executed with connection string: {}", connection_string);
    // Connect to a known peer, e.g., our main network address
    peer.connect(connection_string).await;
}

async fn send(connection_string: &str, peer: Peer) {
    println!("Send command executed.");
    // Implement your 'send' logic here
    // Add a dummy transaction and broadcast it to the network
    let transaction = Transaction {
        sender: "Alice".to_string(),
        recipient: "Bob".to_string(),
        amount: 50,
    };
    peer.send_transaction(connection_string, &transaction).await;

}
