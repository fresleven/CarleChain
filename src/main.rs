mod blockchain;
use blockchain::Blockchain;

fn main() {
    let mut blockchain: Blockchain = Blockchain::new();
    let file_path = "/home/vagrant/CarleChain/data/covid.csv".to_string();
    blockchain.csv_to_blockchain(&file_path).unwrap();
}
