use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::log;
use near_sdk::{collections::LookupMap, env, near_bindgen, AccountId};
use sha2::{Digest, Sha256};

pub type ItemHash = String;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Item {
    min_price: u128,
    pub itself: String,
}

impl Item {
    pub fn new(item: &String, min_price: &u128) -> (Self, ItemHash) {
        (
            Self {
                min_price: *min_price,
                itself: item.to_string(),
            },
            get_hash(item),
        )
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Supplier {
    id: AccountId,
    items: LookupMap<ItemHash, Item>,
}

impl Supplier {
    pub fn new() -> Self {
        Self {
            id: env::predecessor_account_id(),
            items: LookupMap::new(b"s"),
        }
    }

    pub fn add_item_to_auction(&mut self, item: &String, min_price: &u128) {
        let converted_item = Item::new(item, min_price);
        log!("item with hash {} has been inserted", converted_item.1);
        self.items.insert(&converted_item.1, &converted_item.0);
    }

    pub fn sell_item(&mut self, item_hash: &ItemHash) -> Option<Item> {
        self.items.remove(&item_hash)
    }

    pub fn contains_item(&self, item_hash: &String) -> bool {
        self.items.contains_key(&item_hash)
    }

    pub fn get_item(&self, item_hash: &ItemHash) -> String {
        match self.items.get(&item_hash) {
            Some(item) => item.itself,
            None => panic!("supplier does not contain item with hash {}", item_hash)
        }
    }
}

fn get_hash(item: &String) -> String {
    format!("{:X}", Sha256::digest(item.as_bytes()))
}

// fn get_hash_from_vector(item: &Vector<u8>) -> String {
//     let mut chunk:[u8; 1024] = [0; 1024];
//     let mut chunk_index = 0;
//     let mut hash = String::from("");

//     for i in 0..item.len() {
//         if i % 1024 == 0 {
//             hash = format!("{}{:X}", hash, Sha256::digest(chunk));
//             chunk_index = 0;
//         }

//         chunk[chunk_index] = item.get(i).unwrap_or_default();
//         chunk_index += 1;
//     }

//     hash
// }

// mod tests {
//     use super::*;

//     #[test]
//     fn test_get_hash() {
//         let test_phrase = "test phrase".as_bytes();
//         let test_phrase_hash =
//             "03725d0a96e114361230a7978eeefa0d646d7656dce5e44ae4e70a4dea5e674c".to_string();

//         assert_eq!(
//             get_hash(test_phrase).to_lowercase(),
//             test_phrase_hash.to_lowercase()
//         );
//     }

//     #[test]
//     fn test_add_item_to_auction() {
//         let mut supplier = Supplier::new();
//         let item: [u8; 2] = [1, 0];
//         supplier.add_item_to_auction(&Item::new(&item), &12u64);

//         assert_eq!(
//             true,
//             supplier.contains_item(&get_hash(&item)),
//             "item has not been added to a seller"
//         )
//     }

//     #[test]
//     fn test_sell_item() {
//         let item: [u8; 2] = [1, 0];
//         let min_bet: u64 = 12;

//         let mut supplier = Supplier::new();
//         supplier.add_item_to_auction(&Item::new(&item), &min_bet);

//         match supplier.sell_item(&Item::new(&item)) {
//             Some(min_bet_of_selled_item) => assert_eq!(
//                 min_bet_of_selled_item, min_bet,
//                 "wrong item has been selled"
//             ),
//             None => panic!("suppliyer still contains item after a sell"),
//         }
//     }
// }
