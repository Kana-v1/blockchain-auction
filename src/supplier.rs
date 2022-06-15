use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::Balance;
use near_sdk::{collections::UnorderedMap, env, near_bindgen, AccountId};
use sha2::{Digest, Sha256};

use crate::helper::Helper;

pub type ItemHash = String;

pub const DEFAULT_MIN_BID: u128 = 1;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Item {
    min_bid: u128,
    pub itself: String,
}

impl Item {
    pub fn new(item: &String, min_price: &u128) -> (Self, ItemHash) {
        (
            Self {
                min_bid: *min_price,
                itself: item.to_string(),
            },
            get_hash(item),
        )
    }
}

/// Seller that supplies items to an auction
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Supplier {
    pub id: AccountId,
    pub items: UnorderedMap<ItemHash, Item>,
}

impl Supplier {
    pub fn new(helper: &mut Helper) -> Self {
        Self {
            id: env::predecessor_account_id(),
            items: UnorderedMap::new(helper.generate_collection_id()),
        }
    }

    /// add item to an supplier's internal list
    ///
    /// # Arguments
    ///
    /// * `item` - represent of an item in a string format
    /// * `min_price` - minimal price that buyers can. Will be changed to 1 yocto token if 0
    pub fn add_item_to_auction(&mut self, item: &String, min_price: &u128) {
        let correct_min_price = if DEFAULT_MIN_BID < *min_price {
            *min_price
        } else {
            DEFAULT_MIN_BID
        };

        let (item, item_hash) = Item::new(item, &correct_min_price);
        self.items.insert(&item_hash, &item);
    }

    /// remove item from the supplier's internal list
    ///
    /// # Arguments
    /// * `item_hash` - hash calculated from an item through the SHA256 algorithm
    pub fn sell_item(&mut self, item_hash: &ItemHash) -> Option<Item> {
        self.items.remove(&item_hash)
    }

    /// check if selled has added item with such hash to an auction
    ///
    /// # Arguments
    ///
    /// * `item_hash` - hash calculated from an item through the SHA256 algorithm
    pub fn contains_item(&self, item_hash: &String) -> bool {
        self.items.get(&item_hash).is_some()
    }

    /// return item that has been added to an auction
    ///
    /// # Arguments
    ///
    /// * `item_hash` - hash calculated from an item through the SHA256 algorithm
    ///
    ///  # Panics
    ///
    ///  * Supplier has not added item with such hash
    pub fn get_item(&self, item_hash: &ItemHash) -> String {
        match self.items.get(&item_hash) {
            Some(item) => item.itself,
            None => panic!("supplier does not contain item with hash {}", item_hash),
        }
    }

    /// check if deposit is bigger than item's min bid
    ///
    ///  # Arguments
    /// * `item_hash` - hash calculated from an item through the SHA256 algorithm
    /// * `deposit` - attached deposit in yocto tokens
    pub fn bid_can_be_done(&mut self, item_hash: &String, deposit: &Balance) -> (bool, Balance) {
        match self.items.get(item_hash) {
            Some(item) => {
                if item.min_bid > *deposit {
                    return (false, item.min_bid);
                }

                (true, item.min_bid)
            }
            None => (true, 0),
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
        supplier.add_item_to_auction(&item.itself, &item.min_bid);

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
        supplier.add_item_to_auction(&item.itself, &item.min_bid);

        match supplier.sell_item(&item_hash) {
            Some(item) => {
                assert_eq!(item.min_bid, min_bet, "wrong item has been selled");
                assert_eq!(item.itself, item.itself, "wrong item has been selled");
            }

            None => panic!("suppliyer still contains item after a sell"),
        }
    }
}
