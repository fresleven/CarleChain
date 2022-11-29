extern crate hex;

use std::str;
use log::error;
use chrono::Utc;
use sha2::{Sha256, Digest};
use rand::prelude::*;
use serde::{Serialize, Deserialize};

const DIFFICULTY_PREFIX: &str = "000";

#[derive(Debug)]
pub enum BlockError {
    InvalidPreviousHash,
    InvalidPatient,
    InvalidPrefixHash,
    InvalidID,
    IncorrectHash,
}

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

    pub fn add_patient(&mut self, patient_id: u64, age: u64, patient_name: String) {
        self.add_patient_struct(Patient{patient_id, age, patient_name})
    }

    fn add_patient_struct(&mut self, patient: Patient) {
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

    pub fn create_block(&mut self, patient: Patient) -> Block {
        let id = self.blocks.last().expect("Blockchain is not empty").id + 1;
        let timestamp = Utc::now().timestamp();
        let patient_info = patient;
        let previous_hash = &self.blocks.last().expect("Blockchain is not empty").hash;
        let hash = generate_hash(id, previous_hash.clone(), timestamp);
        let nonce = generate_nonce();
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
        } else if block.id -1 != curr_last_block.id {
            return Err(BlockError::InvalidID);
        } else if hex::encode(generate_hash(block.id, block.previous_hash.clone(), block.timestamp)) != block.hash {
            return Err(BlockError::IncorrectHash);
        } else if block.patient_info.patient_name == "" {
            return Err(BlockError::InvalidPatient);
        }
        return Ok(true);
    }

    fn validate_chain(&mut self, chain: &[Block]) -> bool {
        //impl validation algorithm for chain using validate_block
        return true;
    }

    fn mine_block(id: u64, timestamp: i64, previous_hash: &str, patient: Patient) {
        //implement mining algorithm (need nonce)
    }
}

fn generate_hash(id: u64, previous_hash: String, timestamp: i64) -> String {
    //Impl SHA 256 algorithm
    let data = serde_json::json!({
        "id": id, 
        "previous_hash": previous_hash,
        "timestamp": timestamp,
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    return str::from_utf8(&hasher.finalize().as_slice().to_owned()).unwrap().to_string();
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
    let mut rng = rand::thread_rng();
    
    let random_number_64: u64 = rng.gen();
    return random_number_64;
}