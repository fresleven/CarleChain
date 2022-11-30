extern crate hex;
extern crate csv;

use std::str;
use log::error;
use chrono::Utc;
use sha2::{Sha256, Digest};
use rand::prelude::*;
use serde::{Serialize, Deserialize};
use std::error::Error;

const DIFFICULTY_PREFIX: &str = "00";

#[derive(Debug)]
pub enum BlockError {
    InvalidPreviousHash,
    InvalidPatient,
    InvalidPrefixHash,
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
    //data for each hospital patient
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
            // println!("{}, {:?}, {:?}", counter, record[8].parse::<u64>().unwrap(), record[0].to_string());
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
        let nonce = generate_nonce();
        let hash = generate_hash(id, previous_hash.clone(), timestamp, nonce, patient_info.patient_name.clone());
        // println!("{}, {}, {:?}, {}, {}, {},", id, timestamp, patient_info, previous_hash, hash, nonce);
        Block {id, hash, previous_hash : previous_hash.clone(), timestamp, nonce, patient_info}
    }

    fn try_add_block(&mut self, block: Block, curr_last_block: Block) {
        //Need to validate block hash/nonce/id is correct
        let res: Result<bool, BlockError> = self.validate_block(&block, &curr_last_block);
        if res.is_ok() {
            self.blocks.push(block);
        } else {
            match res {
                Err(BlockError::InvalidPreviousHash) => error!("block with id: {} has wrong previous hash", block.id),
                Err(BlockError::InvalidPatient) => error!("block with id: {} has invalid patient information", block.id),
                Err(BlockError::InvalidPrefixHash) => error!("block with id: {} has invalid prefix in the hash", block.id),
                Err(BlockError::IncorrectHash) => error!("block with id: {} has wrong incorrect hash", block.id),
                Err(BlockError::InvalidID) => error!("block with id: {} has wrong invalid ID", block.id),
                _ => error!("Something went terribly wrong!")
            }
        }
    }

    fn validate_block(&mut self, block: &Block, curr_last_block: &Block) -> Result<bool, BlockError> {
        if curr_last_block.hash != block.previous_hash{
            return Err(BlockError::InvalidPreviousHash);
        } else if !hash_to_binary(&hex::decode(&block.hash).unwrap()).starts_with(DIFFICULTY_PREFIX) {
            return Err(BlockError::InvalidPrefixHash);
        } else if block.id - 1 != curr_last_block.id {
            return Err(BlockError::InvalidID);
        } else if hex::encode(generate_hash(block.id, block.previous_hash.clone(), block.timestamp, block.nonce, block.patient_info.patient_name.clone())) != block.hash {
            return Err(BlockError::IncorrectHash);
        } else if block.patient_info.patient_name == "" {
            return Err(BlockError::InvalidPatient);
        }
        return Ok(true);
    }

    fn validate_chain(&mut self, chain: &[Block]) -> bool {
        //impl validation algorithm for chain using validate_block
        for i in 1..chain.len() {
            let curr = chain.get(i).expect("in range");
            let prev = chain.get(i -1).expect("in range");
            if self.validate_block(&curr, &prev).is_err() {
                return false;
            }
        }
        return true;
    }

    fn mine_block(id: u64, timestamp: i64, previous_hash: &str, patient_name: &str) -> (u64, String) {
        let mut nonce = generate_nonce();
        loop {
            let hash = generate_hash(id, previous_hash.to_string(), timestamp, nonce, patient_name.to_string());
            let binary_hash = hash_to_binary(&hash.as_bytes());
            if binary_hash.starts_with(DIFFICULTY_PREFIX) {
                return (nonce, hex::encode(hash));
            }
            nonce = generate_nonce();
        }
    }
}

fn generate_hash(id: u64, previous_hash: String, timestamp: i64, nonce: u64, patient_name: String) -> String {
    //Impl SHA 256 algorithm
    let data = serde_json::json!({
        "id": id, 
        "previous_hash": previous_hash,
        "timestamp": timestamp,
        "nonce": nonce,
        "patient_name": patient_name
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    return hex::encode(hasher.finalize().as_slice().to_owned());
}

fn hash_to_binary(curr_hash: &[u8]) -> String {
    let mut binary: String = "".to_string();
    for character in curr_hash {
        binary += &format!("{:b}", character);
    }
    return binary;
}

fn generate_nonce() -> u64 {
    //Impl random num generator
    let mut rng: ThreadRng = rand::thread_rng();
    
    let random_number_64: u64 = rng.gen();
    return random_number_64;
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
    fn hash_to_binary_test() { // I'll leave this to Ayush to implement
        let hash_1: String = generate_hash(000, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9".to_string(), 0, 0, "Robert IV".to_string());
        let hash_2: String = generate_hash(123, "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(), 4, 4, "Robert V".to_string());
        assert_eq!(hash_to_binary(&hash_1.as_bytes()), "1110111110111111101111011010000100001011101111101111111011110110010011110111101011011110101001110111110111111101111011110111110111111101111011101000110000111111001110111100111110111110111111101111011101101101110111110111111101111011001001101111111001010101110011110111110111111101111011110111110111111101111011101010111011111011111110111101101111011101111101111111011110111001111110".to_string());
        assert_eq!(hash_to_binary(&hash_2.as_bytes()), "1110111110111111101111011010111101011111011111011111110111101101110111110111111101111011110111110111111101111011110111110111111101111011110111110111111101111011000111001111011111011111110111101111110111101111101111111011110111110100011101111101111111011110111101111101111111011110111011001110111110111111101111011110111110111111101111011110111110111111101111011110111110111111101111011110111110111111101111011111011111011111110111101101111110111110111111101111011010011111111000".to_string());
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