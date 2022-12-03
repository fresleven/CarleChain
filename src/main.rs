mod blockchain;
use blockchain::Blockchain;

fn main() {
    let mut blockchain: Blockchain = Blockchain::new();
    let file_path = "/mnt/c/users/lukep/onedrive/documents/rust128/final/data/covid.csv".to_string();
    blockchain.csv_to_blockchain_range(&file_path, 2, 4).unwrap();
    println!("{}", blockchain.validate_chain());
    // blockchain.csv_to_blockchain(&file_path).unwrap();
}
