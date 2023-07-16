use std::{collections::HashMap, error::Error};
use super::Storage;

impl Storage {
    pub fn get_pairs(&self, root_hash: &[u8;32]) -> Result<HashMap<[u8;32], Vec<u8>>, Box<dyn Error>> {
        todo!()
    }
}