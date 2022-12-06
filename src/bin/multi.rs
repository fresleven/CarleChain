mod lib;

use crate::lib::blockchain::Blockchain;
use crate::lib::logreg;

fn main() {
    let mut blockchain: Blockchain = Blockchain::new();
    let file_path = "data/covid.csv".to_string();
    blockchain.csv_to_blockchain(&file_path).unwrap();
    
    if blockchain.validate_chain() {
        println!("\n\n✔️  VALIDATED BLOCKCHAIN\n");
    } else {
        println!("\n\n❌ BROKEN BLOCKCHAIN\n");
    }
    
    let reg_coeffs = blockchain.run_regression();
    println!("{:?}", reg_coeffs);
}