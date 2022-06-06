use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{collections::LookupMap, env, near_bindgen, AccountId};
use sha2::{Digest, Sha256};

use crate::helper::Helper;

pub type ItemHash = String;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Item {
    min_bet: u128,
    pub itself: String,
}

impl Item {
    pub fn new(item: &String, min_price: &u128) -> (Self, ItemHash) {
        (
            Self {
                min_bet: *min_price,
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
    pub fn new(helper: &mut Helper) -> Self {
        Self {
            id: env::predecessor_account_id(),
            items: LookupMap::new(helper.generate_collection_id()),
        }
    }

    pub fn add_item_to_auction(&mut self, item: &String, min_price: &u128) {
        let (item, item_hash) = Item::new(item, min_price);
        self.items.insert(&item_hash, &item);
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
            None => panic!("supplier does not contain item with hash {}", item_hash),
        }
    }
}

fn get_hash(item: &String) -> String {
    format!("{:X}", Sha256::digest(item.as_bytes()))
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hash() {
        let test_phrase = "test phrase".to_string();
        let test_phrase_hash =
            "03725d0a96e114361230a7978eeefa0d646d7656dce5e44ae4e70a4dea5e674c".to_string();

        assert_eq!(
            get_hash(&test_phrase).to_lowercase(),
            test_phrase_hash.to_lowercase()
        );
    }

    #[test]
    fn test_add_item_to_auction() {
        let mut supplier = Supplier::new(&mut Helper::new());
        let (item, item_hash) = Item::new(&"test_item".to_string(), &12u128);
        supplier.add_item_to_auction(&item.itself, &item.min_bet);

        assert_eq!(
            true,
            supplier.contains_item(&item_hash),
            "item has not been added to a seller"
        )
    }

    #[test]
    fn test_sell_item() {
        let min_bet = 12u128;

        let (item, item_hash) = Item::new(&"test_item".to_string(), &12u128);

        let mut supplier = Supplier::new(&mut Helper::new());
        supplier.add_item_to_auction(&item.itself, &item.min_bet);

        match supplier.sell_item(&item_hash) {
            Some(item) => {
                assert_eq!(item.min_bet, min_bet, "wrong item has been selled");
                assert_eq!(item.itself, item.itself, "wrong item has been selled");
            }

            None => panic!("suppliyer still contains item after a sell"),
        }
    }
}
