mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut blockchain: Blockchain = Blockchain::new();

    blockchain.add_patient(001, 24, "Robert IV".to_string());
}
