mod blockchain;
mod logreg;
use blockchain::Blockchain;

fn main() {
    let mut blockchain: Blockchain = Blockchain::new();
    let file_path = "data/covid.csv".to_string();
    blockchain.csv_to_blockchain_range(&file_path, 2, 4).unwrap();
    println!("{}", blockchain.validate_chain());
    let reg_coeffs = blockchain.run_regression();
    println!("{:?}", reg_coeffs);
    blockchain.csv_to_blockchain(&file_path).unwrap();
}
