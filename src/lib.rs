mod helper;
pub mod supplier;

use std::collections::HashMap;

use helper::Helper;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::{env, PanicOnDefault};
use near_sdk::{near_bindgen, AccountId, Promise};

use crate::supplier::Supplier;

type Item = String;
type Money = u128;
type ItemHash = String;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Bid {
    pub account_id: AccountId,
    pub bid: Money,
}

impl Bid {
    pub fn new(account_id: &AccountId, bid: &Money) -> Self {
        Self {
            account_id: account_id.clone(),
            bid: *bid,
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Exchange {
    pub suppliers: UnorderedMap<AccountId, Supplier>, // who want to sell item
    pub items_and_bids: UnorderedMap<ItemHash, Bid>,  // current bid for each item
    pub users_bids: UnorderedMap<AccountId, Money>, // whole sum of all bids for each user (e.g. user wants to buy item_1 and item_2. He bids item_1 = 1 token, item_2 = 1 token. Sum will be 2 tokens)
    pub winners_items: UnorderedMap<AccountId, Vector<Item>>, // each item winner

    auction_is_open: bool,
    helper: Helper,
}

#[near_bindgen]
impl Exchange {
    #[init]
    pub fn new() -> Self {
        Self {
            suppliers: UnorderedMap::new(b"suppliers".to_vec()),
            items_and_bids: UnorderedMap::new(b"items_and_bids".to_vec()),
            users_bids: UnorderedMap::new(b"users_bids".to_vec()),
            winners_items: UnorderedMap::new(b"winners_items".to_vec()),
            auction_is_open: false,
            helper: Helper::new(),
        }
    }

    pub fn start_new_auction(&mut self) {
        assert!(!self.auction_is_open, "Auction is already opened");
        self.auction_is_open = true;
    }

    #[result_serializer(borsh)]
    pub fn get_items(&mut self) -> Vector<String> {
        self.winners_items
            .get(&env::predecessor_account_id())
            .unwrap_or(Vector::new(self.helper.generate_collection_id()))
    }

    pub fn clear_data(&mut self) {
        self.suppliers.clear();
        self.items_and_bids.clear();
        self.users_bids.clear();
    }

    #[payable]
    pub fn make_bid(&mut self, item_hash: &ItemHash) {
        assert!(self.auction_is_open, "Auction is closed. Try again later");

        assert!(
            !self.does_supplier_make_bid_for_his_item(&item_hash),
            "Supplier can not make bid for his items"
        );

        if let Some(exists_bid) = self.items_and_bids.get(&item_hash) {
            assert!(
                env::attached_deposit() > exists_bid.bid,
                "A bigger bid for this item already exists"
            );
        }

        self.items_and_bids.insert(
            &item_hash,
            &Bid::new(&env::predecessor_account_id(), &env::attached_deposit()),
        );

        let bid = self
            .users_bids
            .get(&env::predecessor_account_id())
            .unwrap_or_default()
            + env::attached_deposit();

        self.users_bids.insert(&env::predecessor_account_id(), &bid);
    }

    fn return_money(&mut self, account_id: &AccountId, amount: &u128) {
        assert!(
            self.users_bids.get(&account_id).is_some(),
            "there is no bid for user {}",
            account_id
        );

        assert!(
            self.users_bids.get(&account_id).unwrap() >= *amount,
            "Can not return {} tokens for user {} because his bid is less",
            account_id,
            amount,
        );

        Promise::new(account_id.clone()).transfer(*amount);
    }

    pub fn produce_auction(&mut self) {
        assert!(self.auction_is_open, "Auction has already been finished");

        self.auction_is_open = false;

        let mut winners = HashMap::<ItemHash, Bid>::new();

        for item_and_bid in self.items_and_bids.iter() {
            winners.insert(item_and_bid.0.clone(), item_and_bid.1.clone());

            let rest_money = self
                .users_bids
                .get(&item_and_bid.1.account_id)
                .unwrap_or(item_and_bid.1.bid)
                - item_and_bid.1.bid; // if user won in an auction and loosed in another auction then we have to return money that he spent in the second auction

            self.users_bids
                .insert(&item_and_bid.1.account_id, &rest_money);
        }

        for winner in winners.iter() {
            self.produce_exchange(&winner.1.account_id, &winner.0);
        }

        for user_bid in self.users_bids.iter() {
            Promise::new(user_bid.0.clone()).transfer(user_bid.1.clone());
        }

        self.clear_data();
    }

    pub fn add_item_to_auction(&mut self, item: &Item, min_bid: &u128) {
        assert!(self.auction_is_open, "Auction is closed. Try again later");

        match self.suppliers.get(&env::predecessor_account_id()) {
            Some(mut supplier) => supplier.add_item_to_auction(&item, &min_bid),
            None => {
                let mut supplier = Supplier::new(&mut self.helper);
                supplier.add_item_to_auction(&item, &min_bid);
                self.suppliers
                    .insert(&env::predecessor_account_id(), &supplier);
            }
        }
    }

    fn produce_exchange(&mut self, winner: &AccountId, item: &ItemHash) {
        for mut supplier in self.suppliers.iter() {
            match supplier.1.sell_item(&item) {
                Some(selled_item) => {
                    match self.winners_items.get(&winner) {
                        Some(mut items) => {
                            items.push(&selled_item.itself);
                            self.winners_items.insert(&winner, &items);
                        }

                        None => {
                            let mut v: Vector<Item> =
                                Vector::new(self.helper.generate_collection_id());
                            v.push(&selled_item.itself);

                            self.winners_items.insert(&winner, &v);
                        }
                    }

                    Promise::new(supplier.0.clone()).transfer(
                        self.items_and_bids
                            .get(&item)
                            .unwrap_or(Bid::new(&env::predecessor_account_id(), &0))
                            .bid,
                    );

                    return;
                }

                _ => {}
            };
        }

        // suppliers don't contain item if we got here
        self.return_money(
            &winner,
            &self
                .items_and_bids
                .get(&item)
                .unwrap_or(Bid::new(&env::predecessor_account_id(), &0))
                .bid,
        );
    }

    fn does_supplier_make_bid_for_his_item(&self, item_hash: &ItemHash) -> bool {
        match self.suppliers.get(&env::predecessor_account_id()) {
            None => false,
            Some(supplier) => supplier.contains_item(&item_hash),
        }
    }

    // FOR TEST PURPOSES
    pub fn add_test_item(&mut self) {
        self.add_item_to_auction(&String::from("test_item"), &0)
    }
}

#[allow(unused_imports)]
#[allow(dead_code)]
mod tests {
    use super::*;

    fn get_acc_id() -> AccountId {
        AccountId::try_from("bob.near".to_string()).unwrap()
    }

    #[test]
    #[should_panic]
    fn test_start_started_auction() {
        let mut exchange = Exchange::new();
        exchange.start_new_auction();
        exchange.start_new_auction();
    }

    #[test]
    fn test_get_items() {
        let mut exchange = Exchange::new();

        let item = "test_item".to_string();
        let mut items = Vector::<String>::new(b"i");
        items.push(&item);

        exchange.winners_items.insert(&get_acc_id(), &items);

        assert_eq!(exchange.get_items().len(), 1, "invalid amount of items")
    }

    #[test]
    fn test_clear_data() {
        let mut exchange = Exchange::new();

        let mut items = Vector::<String>::new(b"i");
        items.push(&"item".to_string());

        exchange
            .suppliers
            .insert(&&get_acc_id(), &Supplier::new(&mut Helper::new()));
        exchange
            .items_and_bids
            .insert(&"test_key".to_string(), &Bid::new(&get_acc_id(), &10u128));
        exchange.users_bids.insert(&get_acc_id(), &10u128);
        exchange.winners_items.insert(&get_acc_id(), &items);

        exchange.clear_data();

        assert_eq!(exchange.suppliers.len(), 0);
        assert_eq!(exchange.items_and_bids.len(), 0);
        assert_eq!(exchange.users_bids.len(), 0);

        assert_eq!(exchange.winners_items.len(), 1);
    }

    #[test]
    fn test_make_bid() {
        let mut exchange = Exchange::new();
        exchange.start_new_auction();

        let hash = "hash".to_string();

        exchange.make_bid(&hash);

        assert!(
            exchange.items_and_bids.get(&hash).is_some(),
            "item does not have new bid"
        );
        assert!(
            exchange.users_bids.get(&get_acc_id()).is_some(),
            "user does not have new bid"
        );
    }

    #[test]
    #[should_panic]
    fn test_make_same_bids() {
        let mut exchange = Exchange::new();
        exchange.start_new_auction();

        let hash = "hash".to_string();

        exchange.make_bid(&hash);
        exchange.make_bid(&hash);
    }

    #[test]
    #[should_panic]
    fn test_add_tem_to_closed_auction() {
        let mut exchange = Exchange::new();
        exchange.add_item_to_auction(&"test_item".to_string(), &10u128);
    }

    #[test]
    fn test_add_item_to_auction() {
        let mut exchange = Exchange::new();
        exchange.start_new_auction();

        exchange.add_item_to_auction(&"test_item".to_string(), &10u128);

        assert_eq!(
            exchange.suppliers.len(),
            1,
            "invalid number of suppliers. Expected 1, actual: {}",
            exchange.suppliers.len()
        );
    }

    #[test]
    fn test_supplier_can_not_bid_for_his_items() {
        let mut exchange = Exchange::new();
        exchange.start_new_auction();

        let (_, item_hash) = supplier::Item::new(&"test_item".to_string(), &12u128);

        exchange.add_item_to_auction(&"test_item".to_string(), &10u128);

        assert_eq!(
            exchange.does_supplier_make_bid_for_his_item(&item_hash),
            true,
            "supplier is able to bid for his item"
        );
    }
}
