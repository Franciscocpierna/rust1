// src/block.rs



pub mod block {
    // Sua implementação da struct Blockchain e outras definições de código aqui
    use std::time::SystemTime;
    use crypto::digest::Digest;
    use crypto::sha2::Sha256;
    use log::info;
    use bincode::serialize;
    use serde::{Serialize, Deserialize};

    pub type Result<T> = std::result::Result<T, failure::Error>;


    const TARGET_HEXS: usize = 4;


    // Exemplo fictício de definição do tipo Transaction
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Transaction {
        // Defina os campos da transação aqui
    }



    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Block {
        timestamp: u128,
        transactions: String,
        prev_block_hash: String,
        hash: String,
        nonce: i32,
        height: i32,
    }


    impl Block {
        pub fn get_hash(&self) -> String {
            self.hash.clone()
        }

        pub fn get_prev_hash(&self) -> String{
            self.prev_block_hash.clone()
        }

        pub fn new_genesis_block() -> Block {
            Block::new_block(String::from("Genesis Block"), String::new(), 0).unwrap()
        }

        pub fn new_block(data: String, prev_block_hash: String, height: usize) -> Result<Block> {
            let timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_millis();
            let mut block = Block {
                timestamp,
                transactions: data,
                prev_block_hash,
                hash: String::new(),
                nonce: 0,
                height: height as i32,
            };
            block.run_proof_of_work()?;
            Ok(block)
        }

        fn run_proof_of_work(&mut self) -> Result<()> {
            info!("Mining the block");
            while !self.validate()? {
                self.nonce += 1;
            }
            let data: Vec<u8> = self.prepare_hash_data()?;
            let mut hasher = Sha256::new();
            hasher.input(&data[..]);
            self.hash = hasher.result_str();
            Ok(())
        }

        fn validate(&self) -> Result<bool> {
            let data: Vec<u8> = self.prepare_hash_data()?;
            let mut hasher: Sha256 = Sha256::new();
            hasher.input(&data[..]);
            let mut vec1: Vec<u8> = Vec::new();
            vec1.resize(TARGET_HEXS, '0' as u8);
            Ok(&hasher.result_str()[0..TARGET_HEXS] == String::from_utf8(vec1)?)
        }

        fn prepare_hash_data(&self) -> Result<Vec<u8>> {
            let content = (
                self.prev_block_hash.clone(),
                self.transactions.clone(),
                self.timestamp,
                TARGET_HEXS,
                self.nonce,
            );
            let bytes = serialize(&content)?;
            Ok(bytes)
        }
    }

}
