use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env, AccountId};
use near_sdk::collections::LookupMap;

pub type ItemHash = String;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Buyer {
    pub id: AccountId,
    pub interested_in_items: LookupMap<ItemHash, ItemState>
}

impl Buyer {
    pub fn new() -> Self {
        Self {
            id: env::predecessor_account_id(),
            interested_in_items: LookupMap::new(b"b")
        }
    }

    pub fn make_bet(&mut self, item_hash: &ItemHash) {
        self.interested_in_items.insert(&item_hash, &ItemState::MadeBet);
    }

    pub fn win(&mut self, item_hash: &ItemHash) {
        self.interested_in_items.insert(&item_hash, &ItemState::Won);
    }

    pub fn lose(&mut self, item_hash: &ItemHash) {
        self.interested_in_items.insert(&item_hash, &ItemState::Lost);
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub enum ItemState {
    MadeBet,
    Won,
    Lost,
}

