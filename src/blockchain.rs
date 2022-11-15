pub struct Blockchain {
    pub blocks: Vec,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    //data for each hospital patient
    pub patient_id: u64,
    pub age: u64,
    pub patient_name: String,
    pub nonce: u64,
}

impl Blockchain {
    fn new() -> Self {
        Self { blocks: vec![] }
    }
    //starting block, doesn't adhere to same rules
    fn genesis(&mut self) {
        let genesis_block = Block {
            id: 0,
            timestamp: Utc::now().timestamp(),
            previous_hash: String::from("genesis"),
            data: String::from("genesis!"),
            nonce: 2836,
            hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
        };
        self.blocks.push(genesis_block);
    }

}
