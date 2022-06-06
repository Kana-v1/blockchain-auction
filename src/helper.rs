use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Helper {
    next_id_length: u64,
}

impl Helper {
    pub fn new() -> Self {
        Self {
            next_id_length: 0,
        }
    }

    pub fn generate_collection_id(&mut self) -> Vec<u8> {
        let symbols = vec![
            "a", "b", "c", "d", "e", "f", "g", "h", "q", "w", "e", "r", "t", "y", "u", "i", "p",
            "o", "r", "!", "1", "2", "3", "3", "4",
        ];

        let mut collection_id = Vec::<u8>::new();

        let mut j = 0usize;

        for i in 0..self.next_id_length {
            if i as usize / symbols.len() >= 1 {
                j = 0
            }

            collection_id.extend(symbols[j].as_bytes());
        }

        self.next_id_length += 1;

        collection_id
    }
}
