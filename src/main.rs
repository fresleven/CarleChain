mod blockchain;
use blockchain::Blockchain;

fn main() {
    let mut blockchain: Blockchain = Blockchain::new();
    let file_path = "data/covid.csv".to_string();
    //blockchain.csv_to_blockchain_range(&file_path, 0, 563200).unwrap();
    blockchain.csv_to_blockchain(&file_path).unwrap();
    println!("{}", blockchain.validate_chain());
}
