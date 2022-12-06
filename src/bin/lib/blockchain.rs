use csv::StringRecord;

use log::error;
use chrono::Utc;
use sha2::{Sha256, Digest};
use rand::prelude::*;
use serde::{Serialize, Deserialize};
use std::error::Error;

use ndarray::{Array1, Array2};
use crate::logreg::logistic_regression;

use std::sync::{mpsc, mpsc::Receiver};
use std::thread;
use std::thread::JoinHandle;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

const DIFFICULTY_PREFIX: &str = "000";
const NUM_OF_ROWS_COVID: u64 = 566602;

//Structure of encapsulated patient data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Patient {
    pub id: String,
    pub sex: char,
    patient_type: u8,
    entry_date: String,
    date_symptoms: String,
    date_died: String,
    intubed: u8,
    pneumonia: u8,
    age: i64,
    pregnancy: u8,
    diabetes: u8,
    copd: u8,
    asthma: u8,
    inmsupr: u8,
    hypertension: u8,
    other_disease: u8,
    cardiovascular: u8,
    obesity: u8,
    renal_chronic: u8,
    tobacco: u8,
    contact_other_covid: u8,
    covid_res: u64,
    icu: u8,
    if_died: u8,
}

#[derive(Debug)]
//Enum used to validate block
pub enum BlockError {
    InvalidPreviousHash,
    InvalidPatient,
    InvalidID,
    IncorrectHash
}

#[derive(Serialize, Deserialize, Debug, Clone)]
//Single block structure
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub nonce: u64,
    pub patient_info: Patient
}

#[derive(Debug)]
//blocks represents entire ledger
pub struct Blockchain {
    pub blocks: Vec<Block>
}

//Reads string given a vector of iterators to lines in the CSV
pub fn string_reader(records: &Vec<StringRecord>, pb: &ProgressBar, m: &MultiProgress) -> Vec<Patient> {
    let mut patients: Vec<Patient> = Vec::new();
    for i in 0..records.len() {
        let record = &records[i];
        let died: u8 = if record[5].to_string() == "9999-99-99".to_string() { 0 } else { 1 };
        let patient = Patient{id: record[0].parse::<String>().unwrap(), sex: record[1].parse::<char>().unwrap(), patient_type: record[2].parse::<u8>().unwrap(), entry_date: record[3].parse::<String>().unwrap(), date_symptoms: record[4].parse::<String>().unwrap(), date_died: record[5].parse::<String>().unwrap(), intubed: record[6].parse::<u8>().unwrap(), pneumonia: record[7].parse::<u8>().unwrap(), age: record[8].parse::<i64>().unwrap(), pregnancy: record[9].parse::<u8>().unwrap(), diabetes: record[10].parse::<u8>().unwrap(),copd: record[11].parse::<u8>().unwrap(), asthma: record[12].parse::<u8>().unwrap(), inmsupr: record[13].parse::<u8>().unwrap(), hypertension: record[14].parse::<u8>().unwrap(), other_disease: record[15].parse::<u8>().unwrap(), cardiovascular: record[16].parse::<u8>().unwrap(), obesity: record[17].parse::<u8>().unwrap(), renal_chronic: record[18].parse::<u8>().unwrap(), tobacco: record[19].parse::<u8>().unwrap(), contact_other_covid: record[20].parse::<u8>().unwrap(), covid_res: record[21].parse::<u64>().expect(&("COVID_RES record is an integer".to_string() + &record[21].to_string())), icu: record[22].parse::<u8>().expect(&("ICU record is an integer".to_string() + &record[22].to_string())), if_died: died};
        patients.push(patient);
        pb.set_message(format!("item #{}", i + 1));
        pb.inc(1);
    }
    m.println("").unwrap();
    pb.finish_with_message("done");
    return patients;
}

#[allow(dead_code)]
impl Blockchain {
    pub fn new() -> Self {
        return Self { blocks: vec![] };
    }

    //Given number of chunks, divides up csv lines accordingly
    pub fn split_into_chunks(&self, file_path_: &String, num_chunks: usize) 
            -> Vec<Vec<csv::StringRecord>> {
        let mut reader = csv::Reader::from_path(file_path_).unwrap();
        let mut chunks: Vec<Vec<csv::StringRecord>> = vec![Vec::new(); num_chunks];
        for (idx, record) in reader.records().enumerate() {
            let chunk_idx = idx % num_chunks;
            chunks.get_mut(chunk_idx).unwrap().push(record.unwrap().clone());
        }
        return chunks;
    }

    //Returns a vector of join handles with each handle assigned a chunk
    pub fn multi_threaded_reader(&self, file_path_: &String, num_chunks: usize) -> (Vec<JoinHandle<()>>, Receiver<Vec<Patient>>) {
        let (tx,rx) = mpsc::channel();
        let mut handles = Vec::new();
        let chunks = self.split_into_chunks(file_path_, num_chunks);
        
        let m = MultiProgress::new();
        let sty = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        ).unwrap()
        .progress_chars("##-");
        let mut pb_vec: Vec<ProgressBar> = Vec::new();
        for i in 0..num_chunks {
            let pb = m.add(ProgressBar::new(chunks.get(i).unwrap().len().try_into().unwrap()));
            pb.set_style(sty.clone());
            pb_vec.push(pb);
        }

        println!("\nLOADING IN PATIENTS");
        for i in 0..num_chunks {
            let owned_chunk = chunks.get(i).unwrap().clone();
            let tx_clone = tx.clone();
            let pb_clone = pb_vec[i].clone();
            let m_clone = m.clone();
            let h = thread::spawn(move || {
                let result = string_reader(&owned_chunk, &pb_clone, &m_clone);
                tx_clone.send(result).unwrap();
            });
            handles.push(h);
        }
        m.println("").unwrap();
        return (handles, rx);
    }

    //Each thread returns a vector of patients after reading the csv in parallel. Adds each of the patients
    pub fn thread_reducer(&mut self, receivers: (Vec<JoinHandle<()>>, Receiver<Vec<Patient>>)) {
        let (_, results) = receivers;
        let pb = ProgressBar::new(NUM_OF_ROWS_COVID);
        let sty = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:60.green} {pos:>7}/{len:7} {msg}",
        ).unwrap();
        pb.set_style(sty);
        
        while let Ok(patients) = results.recv() {
            for patient in patients {
                self.add_patient_struct(patient);
                pb.inc(1);
            }
        }
        pb.finish_with_message("done");
    }

    //Calls threading process
    pub fn csv_to_blockchain(&mut self, file_path_: &String) -> Result<(), Box<dyn Error>> {
        if !self.blocks.is_empty() { panic!("cannot call on non-empty blockchain!"); }
        let num_chunks: usize = 8;
        self.thread_reducer(self.multi_threaded_reader(file_path_, num_chunks));
        return Ok(());
    }

    //Single threaded approach to reading only a slice of the CSV.
    pub fn csv_to_blockchain_range(&mut self, file_path_: &String, start: usize, length: usize) -> Result<(), Box<dyn Error>> {    
        if length == 0 {
            panic!("NEED TO HAVE AT LEAST ONE BLOCK");
        }
        if length > NUM_OF_ROWS_COVID as usize || 
            start >= NUM_OF_ROWS_COVID as usize || 
            length + start > NUM_OF_ROWS_COVID as usize {
            panic!("INVALID START OR LENGTH! MAX START + LENGTH = {}", NUM_OF_ROWS_COVID);
        }
        
        let mut reader = csv::Reader::from_path(file_path_)?;
        let slice = &reader.records().collect::<Vec<Result<csv::StringRecord, csv::Error>>>()[start..start + length];
        let pb = ProgressBar::new(length.try_into().unwrap());
        let sty = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:60.green} {pos:>7}/{len:7} {msg}",
        ).unwrap();
        pb.set_style(sty);

        println!("\nCREATING BLOCKCHAIN FROM PATIENT {} TO {}", start, start + length - 1);
        for rec in slice {
            match rec {
                Ok(record) => {
                    let died: u8 = if record[5].to_string() == "9999-99-99".to_string() { 0 } else { 1 };
                    self.add_patient(record[0].parse::<String>().unwrap(), record[1].parse::<char>().unwrap(), record[2].parse::<u8>().unwrap(), record[3].parse::<String>().unwrap(), record[4].parse::<String>().unwrap(), record[5].parse::<String>().unwrap(), record[6].parse::<u8>().unwrap(), record[7].parse::<u8>().unwrap(), record[8].parse::<i64>().unwrap(), record[9].parse::<u8>().unwrap(), record[10].parse::<u8>().unwrap(), record[11].parse::<u8>().unwrap(), record[12].parse::<u8>().unwrap(), record[13].parse::<u8>().unwrap(), record[14].parse::<u8>().unwrap(), record[15].parse::<u8>().unwrap(), record[16].parse::<u8>().unwrap(), record[17].parse::<u8>().unwrap(), record[18].parse::<u8>().unwrap(), record[19].parse::<u8>().unwrap(), record[20].parse::<u8>().unwrap(), record[21].parse::<u64>().unwrap(), record[22].parse::<u8>().unwrap(), died);
                    pb.inc(1);
                },
                Err(_) => panic!("an error occurred")
            }
        }
        pb.finish_with_message("done");
        return Ok(());
    }

    //Creates the first block in the blockchain and sets initialize hash
    fn genesis(&mut self, patient: Patient) {
        let id = 0;
        let timestamp = Utc::now().timestamp();
        let previous_hash = String::from("genesis");
        let patient_info = patient;
        let nonce = 2836;
        let hash = "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string();

        let genesis_block = Block {
            id: id,
            timestamp: timestamp,
            previous_hash: previous_hash,
            patient_info: patient_info,
            nonce: nonce,
            hash: hash
        };
        self.blocks.push(genesis_block);
    }

    //Given the fields for a patient, creates a patient of type Patient and adds them to the blockchain
    pub fn add_patient(&mut self, id: String, sex: char, patient_type: u8, entry_date: String, date_symptoms: String, date_died: String, intubed: u8, pneumonia: u8, age: i64, pregnancy: u8, diabetes: u8, copd: u8, asthma: u8, inmsupr: u8, hypertension: u8, other_disease: u8, cardiovascular: u8, obesity: u8, renal_chronic: u8, tobacco: u8, contact_other_covid: u8, covid_res: u64, icu: u8, if_died: u8) {
        self.add_patient_struct(Patient{id,sex,patient_type,entry_date,date_symptoms,date_died,intubed,pneumonia,age,pregnancy,diabetes,copd,asthma,inmsupr,hypertension,other_disease,cardiovascular,obesity,renal_chronic,tobacco,contact_other_covid,covid_res,icu, if_died});
    }

    //Adds patient to blockchain
    pub fn add_patient_struct(&mut self, patient: Patient) {
        if self.blocks.is_empty() {
            self.genesis(patient);
        }
        else {
            self.add_patient_nonempty(patient);
        }
    }

    //Adds a patient when the blockchain is not empty
    fn add_patient_nonempty(&mut self, patient: Patient) {
        let block: Block = self.create_block(patient);
        let curr_last_block: Block = self.blocks.last().unwrap().clone();
        self.try_add_block(block, curr_last_block);
    }

    //Creates a block and ensures the block is mined
    fn create_block(&mut self, patient: Patient) -> Block {
        let id = self.blocks.last().expect("Blockchain is not empty").id + 1;
        let timestamp = Utc::now().timestamp();
        let patient_info = patient;
        let previous_hash = &self.blocks.last().expect("Blockchain is not empty").hash;
        let (nonce, hash) = mine_block(id, timestamp, previous_hash.as_str(), patient_info.id.clone());
        return Block {id, hash, previous_hash : previous_hash.clone(), timestamp, nonce, patient_info};
    }

    //Adds a block given there is no issue with validation
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

    //Validates a block by checking id, previous hash, patient data, and current hash
    fn validate_block(&mut self, block: &Block, curr_last_block: &Block) -> Result<bool, BlockError> {
        if curr_last_block.hash != block.previous_hash {
            return Err(BlockError::InvalidPreviousHash);
        } else if block.id - 1 != curr_last_block.id {
            return Err(BlockError::InvalidID);
        } else if generate_hash(block.id, block.previous_hash.clone(), block.timestamp, block.nonce, block.patient_info.id.clone()) != block.hash {
            return Err(BlockError::IncorrectHash);
        } else if block.patient_info.id.is_empty() {
            return Err(BlockError::InvalidPatient);
        }
        return Ok(true);
    }

    //Validates each block on the chain
    pub fn validate_chain(&mut self) -> bool {
        let chain = self.blocks.clone();
        for i in 1..chain.len() {
            let curr = chain.get(i).expect("in range");
            let prev = chain.get(i - 1).expect("in range");
            if self.validate_block(&curr, &prev).is_err() {
                return false;
            }
        }
        return true;
    }

    pub fn run_regression(&mut self) -> Array1<f64> {
        let chain = self.blocks.clone();
        let rows = chain.len();
        if rows < 6 {
            panic!("too few rows to run regression!");
        }
        let mut x_arr = Array2::<f64>::zeros((rows, 6));
        let mut y_arr = Array1::<f64>::zeros(rows);
        for (idx, block) in chain.iter().enumerate() {
            let patient = &block.patient_info;
            y_arr[idx] = patient.if_died as f64;
            x_arr[[idx,0]] = 1.0;
            x_arr[[idx,1]] = if patient.sex == '2' { 1.0 } else { 0.0 };
            x_arr[[idx,2]] = (patient.pneumonia % 2) as f64;
            x_arr[[idx,3]] = (patient.diabetes % 2) as f64;
            x_arr[[idx,4]] = (patient.hypertension % 2) as f64;
            x_arr[[idx,5]] = (patient.tobacco % 2) as f64;
        }
        return logistic_regression(&x_arr, &y_arr);
    }

    // println!("{:?},\nPrevious Hash: {},\nHash: {},\nNonce: {}\n", patient_info, previous_hash, hash, nonce);
}

//Uses RNG to generate nonce value
fn generate_nonce() -> u64 {
    let mut rng: ThreadRng = rand::thread_rng();
    
    let random_number_64: u64 = rng.gen();
    return random_number_64;
}

//Generates a hash using SHA256 impl
fn generate_hash(id: u64, previous_hash: String, timestamp: i64, nonce: u64, patient_id: String) -> String {
    let data = serde_json::json!({
        "id": id, 
        "previous_hash": previous_hash,
        "nonce": nonce,
        "timestamp": timestamp,
        "patient_id": patient_id
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    return hex::encode(hasher.finalize().as_slice().to_owned());
}

//Mines a block and returns nonce and hash for specified difficulty 
fn mine_block(id: u64, timestamp: i64, previous_hash: &str, patient_id: String) -> (u64, String) {
    let mut nonce = generate_nonce();
    loop {
        let hash = generate_hash(id, previous_hash.to_string(), timestamp, nonce, patient_id.clone());
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
        assert_eq!(generate_hash(001, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9".to_string(), 1669749953, 0, "16169f".to_string()), 
            generate_hash(001, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9".to_string(), 1669749953, 0, "16169f".to_string()));
        assert_ne!(generate_hash(001, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9".to_string(), 1669749953, 0, "16169f".to_string()), 
            generate_hash(002, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9".to_string(), 1669749953, 0, "16169f".to_string()));
        assert_ne!(generate_hash(000, "".to_string(), 0, 0, "16169f".to_string()), generate_hash(001, "a".to_string(), 1, 0, "16169f".to_string()));
    }

    #[test]
    fn generate_nonce_test() {
        let mut nonces: HashSet<u64> = HashSet::new();
        for _i in 0..1000 {
            nonces.insert(generate_nonce());
        }
        assert_eq!(nonces.len(), 1000);
    }
}