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

pub struct Patient {
    pub patient_id: u64,
    pub age: u64,
    pub patient_name: String,
}
impl Blockchain {
    fn new() -> Self {
        Self { blocks: vec![] }
    }
    fn add_patient(&mut self, patient_id: u64, age: u64, patient_name: u64) {
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
        self.try_add_block(block)
    }
    fn create_block(&mut self, patient: Patient) -> Block {
        let id = self.blocks.last().expect("Blockchain is not empty").id + 1;
        let timestamp = Utc::now().timestamp();
        let patient_info = patient;
        let previous_hash = self.blocks.last().expect("Blockchain is not empty").hash;
        let hash = generate_hash(previous_hash);
        let nonce = generate_nonce();
        Block {id, hash, previous_hash, timestamp, nonce, patient_info}
    }
    fn try_add_block(&mut self, block: Block) {
        //Need to validate block hash/nonce/id is correct
    }
}

fn generate_hash(previous_hash: String) -> String {
    //Impl SHA 256 algorithm
}

fn generate_nonce() -> u64 {
    //Impl random num generator
}