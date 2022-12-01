use std::str;
use log::error;
use chrono::Utc;
use sha2::{Sha256, Digest};
use rand::prelude::*;
use serde::{Serialize, Deserialize};
use std::error::Error;

const DIFFICULTY_PREFIX: &str = "00000";

#[derive(Debug)]
pub enum BlockError {
    InvalidPreviousHash,
    InvalidPatient,
    InvalidID,
    IncorrectHash,
}

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub nonce: u64,
    pub patient_info : Patient
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Patient {
    pub patient_id: u64,
    pub age: u64,
    pub patient_name: String,
}

impl Blockchain {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }

    pub fn csv_to_blockchain(&mut self, file_path_: &String) -> Result<(), Box<dyn Error>> {
        let mut reader = csv::Reader::from_path(file_path_)?;
        let mut counter = 0;
        for result in reader.records() {
            let record = result?;
            self.add_patient(counter, record[8].parse::<u64>().unwrap(), record[0].to_string());
            counter += 1;
        }
        return Ok(());
    }

    pub fn add_patient(&mut self, patient_id: u64, age: u64, patient_name: String) {
        self.add_patient_struct(Patient{patient_id, age, patient_name})
    }

    pub fn add_patient_struct(&mut self, patient: Patient) {
        if self.blocks.is_empty() {
            self.genesis(patient)
        }
        else {
            self.add_patient_nonempty(patient)
        }
    }

    fn genesis(&mut self, patient: Patient) {
        let genesis_block = Block {
            id: 0,
            timestamp: Utc::now().timestamp(),
            previous_hash: String::from("genesis"),
            patient_info: patient,
            nonce: 2836,
            hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
        };
        self.blocks.push(genesis_block);
    }

    fn add_patient_nonempty(&mut self, patient: Patient) {
        let block: Block = self.create_block(patient);
        let curr_last_block: Block = self.blocks.last().unwrap().clone();
        self.try_add_block(block, curr_last_block);
    }

    fn create_block(&mut self, patient: Patient) -> Block {
        let id = self.blocks.last().expect("Blockchain is not empty").id + 1;
        let timestamp = Utc::now().timestamp();
        let patient_info = patient;
        let previous_hash = &self.blocks.last().expect("Blockchain is not empty").hash;
        let (nonce, hash) = mine_block(id, timestamp, previous_hash.as_str(), patient_info.patient_name.as_str());
        println!("{:?},\nPrevious Hash: {},\nHash: {},\nNonce: {}\n", patient_info, previous_hash, hash, nonce);
        return Block {id, hash, previous_hash : previous_hash.clone(), timestamp, nonce, patient_info};
    }

    fn try_add_block(&mut self, block: Block, curr_last_block: Block) {
        let res: Result<bool, BlockError> = self.validate_block(&block, &curr_last_block);
        if res.is_ok() {
            self.blocks.push(block);
        } else {
            match res {
                Err(BlockError::InvalidPreviousHash) => error!("block with id: {} has wrong previous hash", block.id),
                Err(BlockError::InvalidPatient) => error!("block with id: {} has invalid patient information", block.id),
                Err(BlockError::IncorrectHash) => error!("block with id: {} has wrong incorrect hash", block.id),
                Err(BlockError::InvalidID) => error!("block with id: {} has wrong invalid ID", block.id),
                _ => error!("Something went terribly wrong!")
            }
        }
    }

    fn validate_block(&mut self, block: &Block, curr_last_block: &Block) -> Result<bool, BlockError> {
        if curr_last_block.hash != block.previous_hash {
            return Err(BlockError::InvalidPreviousHash);
        } else if block.id - 1 != curr_last_block.id {
            return Err(BlockError::InvalidID);
        } else if generate_hash(block.id, block.previous_hash.clone(), block.timestamp, block.nonce, block.patient_info.patient_name.clone()) != block.hash {
            return Err(BlockError::IncorrectHash);
        } else if block.patient_info.patient_name == "" {
            return Err(BlockError::InvalidPatient);
        }
        return Ok(true);
    }

    fn validate_chain(&mut self, chain: &[Block]) -> bool {
        for i in 1..chain.len() {
            let curr = chain.get(i).expect("in range");
            let prev = chain.get(i - 1).expect("in range");
            if self.validate_block(&curr, &prev).is_err() {
                return false;
            }
        }
        return true;
    }
}

fn generate_hash(id: u64, previous_hash: String, timestamp: i64, nonce: u64, patient_name: String) -> String {
    let data = serde_json::json!({
        "id": id, 
        "previous_hash": previous_hash,
        "nonce": nonce,
        "timestamp": timestamp,
        "patient_name": patient_name
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    return hex::encode(hasher.finalize().as_slice().to_owned());
}

fn generate_nonce() -> u64 {
    let mut rng: ThreadRng = rand::thread_rng();
    
    let random_number_64: u64 = rng.gen();
    return random_number_64;
}

fn mine_block(id: u64, timestamp: i64, previous_hash: &str, patient_name: &str) -> (u64, String) {
    let mut nonce = generate_nonce();
    loop {
        let hash = generate_hash(id, previous_hash.to_string(), timestamp, nonce, patient_name.to_string());
        if hash.starts_with(DIFFICULTY_PREFIX) {
            return (nonce, hash);
        }
        nonce = generate_nonce();
    }
}

#[cfg(test)]

mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_generate_hash() {
        assert_eq!(generate_hash(001, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9".to_string(), 1669749953, 0, "Robert IV".to_string()), 
            generate_hash(001, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9".to_string(), 1669749953, 0, "Robert IV".to_string()));
        assert_ne!(generate_hash(001, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9".to_string(), 1669749953, 0, "Robert IV".to_string()), 
            generate_hash(002, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9".to_string(), 1669749953, 0, "Robert IV".to_string()));
        assert_ne!(generate_hash(000, "".to_string(), 0, 0, "Robert IV".to_string()), generate_hash(001, "a".to_string(), 1, 0, "Robert IV".to_string()));
    }

    #[test]
    fn generate_nonce_test() {
        let mut nonces: HashSet<u64> = HashSet::new();
        for _i in 0..100 {
            nonces.insert(generate_nonce());
        }
        assert_eq!(nonces.len(), 100);
    }

    #[test]
    fn add_patient_test() {
        todo!();
    }

    #[test]
    fn add_patient_struct_test() {
        todo!();
    }

    #[test]
    fn validate_block_test() {
        todo!();
    }

    #[test]
    fn validate_chain_test() {
        todo!();
    }

    #[test]
    fn mining_block_test() {
        todo!();
    }
}