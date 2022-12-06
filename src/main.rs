mod bin;

use crate::bin::lib::blockchain::Blockchain;
use crate::bin::lib::logreg;

#[allow(non_snake_case)]
fn main() {
    let start_patient_idx: usize = 0;
    let length: usize = 50;

    let mut blockchain: Blockchain = Blockchain::new();
    let file_path = "data/covid.csv".to_string();
    blockchain.csv_to_blockchain_range(&file_path, start_patient_idx, length).unwrap(); 

    if blockchain.validate_chain() {
        println!("\n\n✔️  VALIDATED BLOCKCHAIN\n");
    } else {
        println!("\n\n❌ BROKEN BLOCKCHAIN\n");
    }
    
    let reg_coeffs = blockchain.run_regression();
    println!("{:?}", reg_coeffs);
}