#[allow(unused_imports)]
use auction::Bid;
#[allow(unused_imports)]
use near_sdk::collections::UnorderedMap;
use near_sdk::collections::Vector;
#[allow(unused_imports)]
use near_sdk::AccountId;
use near_units::parse_near;
use serde_json::from_str;
use serde_json::Value;
use workspaces::prelude::*;

const SELLER_ACC_ID: &str = "seller";
const WINNER_ACC_ID: &str = "winner";
const LOSER_ACC_ID: &str = "loser";
const WASM_FILEPATH: &str = "./res/auction.wasm";

fn yocto_to_token(n: u128) -> f64 {
    n as f64 / 10u128.pow(24) as f64
}

#[tokio::test]
async fn test_single_participant() {
    /* #region  init*/
    let worker = workspaces::sandbox().await.unwrap();
    let wasm = std::fs::read(WASM_FILEPATH).unwrap();
    let contract = worker.dev_deploy(&wasm).await.unwrap();

    let owner = worker.root_account();

    owner
        .call(&worker, contract.id(), "new")
        .transact()
        .await
        .unwrap();

    owner
        .call(&worker, contract.id(), "start_new_auction")
        .transact()
        .await
        .unwrap();

    let winner = owner
        .create_subaccount(&worker, WINNER_ACC_ID)
        .initial_balance(parse_near!("20 N"))
        .transact()
        .await
        .unwrap()
        .unwrap();

    let seller = owner
        .create_subaccount(&worker, SELLER_ACC_ID)
        .initial_balance(parse_near!("20 N"))
        .transact()
        .await
        .unwrap()
        .unwrap();
    /* #endregion*/

    let args_for_sell: Value = from_str(
        r#"
    {
        "item":"test_item",
        "min_bid": 0
    }"#,
    )
    .unwrap();

    seller
        .call(&worker, contract.id(), "add_item_to_auction")
        .args_json(args_for_sell)
        .unwrap()
        .transact()
        .await
        .unwrap();

    let args_for_bid: Value = from_str(
        r#"{
            "item_hash":"68E5EE009D13B901BBB36D3BB47FC59ACA581D6DB141DA0574287495244A9225"
    }
        "#,
    )
    .unwrap();

    winner
        .call(&worker, contract.id(), "make_bid")
        .args_json(args_for_bid)
        .unwrap()
        .deposit(parse_near!("10 N"))
        .transact()
        .await
        .unwrap();

    owner
        .call(&worker, contract.id(), "produce_auction")
        .transact()
        .await
        .unwrap();

    let winner_items: Vector<String> = winner
        .call(&worker, contract.id(), "get_items")
        .transact()
        .await
        .unwrap()
        .borsh()
        .unwrap();

    assert_eq!(
        winner_items.len(),
        1,
        "Incorrect items amount. Should be 1, actual {}",
        winner_items.len()
    );

    let acc = seller.view_account(&worker).await.unwrap();

    assert_eq!(
        yocto_to_token(acc.balance).ceil(), // much less than 1 token has been burned as gas
        30f64,
        "Seller has invalid amount of money. Should be 20 N, actual: {}",
        acc.balance
    );
}

#[tokio::test]
async fn test_multiple_participants() {
    /* #region  init*/
    let worker = workspaces::sandbox().await.unwrap();
    let wasm = std::fs::read(WASM_FILEPATH).unwrap();
    let contract = worker.dev_deploy(&wasm).await.unwrap();

    let owner = worker.root_account();

    owner
        .call(&worker, contract.id(), "new")
        .transact()
        .await
        .unwrap();

    owner
        .call(&worker, contract.id(), "start_new_auction")
        .transact()
        .await
        .unwrap();

    let winner = owner
        .create_subaccount(&worker, WINNER_ACC_ID)
        .initial_balance(parse_near!("20 N"))
        .transact()
        .await
        .unwrap()
        .unwrap();

    let seller = owner
        .create_subaccount(&worker, SELLER_ACC_ID)
        .initial_balance(parse_near!("20 N"))
        .transact()
        .await
        .unwrap()
        .unwrap();

    let loser = owner
        .create_subaccount(&worker, LOSER_ACC_ID)
        .initial_balance(parse_near!("20 N"))
        .transact()
        .await
        .unwrap()
        .unwrap();
    /* #endregion*/

    let args_for_sell: Value = from_str(
        r#"
    {
        "item":"test_item",
        "min_bid": 0
    }"#,
    )
    .unwrap();

    seller
        .call(&worker, contract.id(), "add_item_to_auction")
        .args_json(args_for_sell)
        .unwrap()
        .transact()
        .await
        .unwrap();

    let args_for_bid: Value = from_str(
        r#"{
            "item_hash":"68E5EE009D13B901BBB36D3BB47FC59ACA581D6DB141DA0574287495244A9225"
    }
        "#,
    )
    .unwrap();

    loser
        .call(&worker, contract.id(), "make_bid")
        .args_json(&args_for_bid)
        .unwrap()
        .deposit(parse_near!("5 N"))
        .transact()
        .await
        .unwrap();

    winner
        .call(&worker, contract.id(), "make_bid")
        .args_json(&args_for_bid)
        .unwrap()
        .deposit(parse_near!("10 N"))
        .transact()
        .await
        .unwrap();

    owner
        .call(&worker, contract.id(), "produce_auction")
        .transact()
        .await
        .unwrap();

    let winner_items: Vector<String> = winner
        .call(&worker, contract.id(), "get_items")
        .transact()
        .await
        .unwrap()
        .borsh()
        .unwrap();

    assert_eq!(
        winner_items.len(),
        1,
        "Incorrect items amount. Should be 1, actual {}",
        winner_items.len()
    );

    let seller_acc = seller.view_account(&worker).await.unwrap();

    assert_eq!(
        yocto_to_token(seller_acc.balance).ceil(),
        30f64,
        "Seller has invalid amount of money. Should be 20 N, actual: {}",
        seller_acc.balance
    );

    let loser_acc = loser.view_account(&worker).await.unwrap();

    assert_eq!(
        yocto_to_token(loser_acc.balance).ceil(),
        20f64,
        "Seller has invalid amount of money. Should be 30 N, actual: {}",
        loser_acc.balance
    )
}

#[tokio::test]
async fn two_two_auctions_in_sequence() {
    /* #region */
    let worker = workspaces::sandbox().await.unwrap();
    let wasm = std::fs::read(WASM_FILEPATH).unwrap();
    let contract = worker.dev_deploy(&wasm).await.unwrap();

    let owner = worker.root_account();

    owner
        .call(&worker, contract.id(), "new")
        .transact()
        .await
        .unwrap();

    let winner = owner
        .create_subaccount(&worker, WINNER_ACC_ID)
        .initial_balance(parse_near!("20 N"))
        .transact()
        .await
        .unwrap()
        .unwrap();

    let seller = owner
        .create_subaccount(&worker, SELLER_ACC_ID)
        .initial_balance(parse_near!("20 N"))
        .transact()
        .await
        .unwrap()
        .unwrap();

    let loser = owner
        .create_subaccount(&worker, LOSER_ACC_ID)
        .initial_balance(parse_near!("20 N"))
        .transact()
        .await
        .unwrap()
        .unwrap();

    owner.call(&worker, contract.id(), "new");

    owner
        .call(&worker, contract.id(), "start_new_auction")
        .transact()
        .await
        .unwrap();

    let args_for_sell: Value = from_str(
        r#"
    {
        "item":"test_item",
        "min_bid": 0
    }"#,
    )
    .unwrap();
    /* #endregion*/

    seller
        .call(&worker, contract.id(), "add_item_to_auction")
        .args_json(args_for_sell.clone())
        .unwrap()
        .transact()
        .await
        .unwrap();

    let args_for_bid: Value = from_str(
        r#"{
            "item_hash":"68E5EE009D13B901BBB36D3BB47FC59ACA581D6DB141DA0574287495244A9225"
    }
        "#,
    )
    .unwrap();

    loser
        .call(&worker, contract.id(), "make_bid")
        .args_json(&args_for_bid)
        .unwrap()
        .deposit(parse_near!("5 N"))
        .transact()
        .await
        .unwrap();

    winner
        .call(&worker, contract.id(), "make_bid")
        .args_json(&args_for_bid)
        .unwrap()
        .deposit(parse_near!("10 N"))
        .transact()
        .await
        .unwrap();

    owner
        .call(&worker, contract.id(), "produce_auction")
        .transact()
        .await
        .unwrap();

    let winner_items: Vector<String> = winner
        .call(&worker, contract.id(), "get_items")
        .transact()
        .await
        .unwrap()
        .borsh()
        .unwrap();

    assert_eq!(
        winner_items.len(),
        1,
        "Incorrect items amount. Should be 1, actual {}",
        winner_items.len()
    );

    let seller_acc = seller.view_account(&worker).await.unwrap();
    let loser_acc = loser.view_account(&worker).await.unwrap();

    assert_eq!(
        yocto_to_token(seller_acc.balance).ceil(),
        30f64,
        "Seller has invalid amount of money. Should be 30 N, actual: {}",
        seller_acc.balance
    );

    assert_eq!(
        yocto_to_token(loser_acc.balance).ceil(),
        20f64,
        "Seller has invalid amount of money. Should be 20 N, actual: {}",
        loser_acc.balance
    );

    owner
        .call(&worker, contract.id(), "start_new_auction")
        .transact()
        .await
        .unwrap();

    seller
        .call(&worker, contract.id(), "add_item_to_auction")
        .args_json(args_for_sell)
        .unwrap()
        .transact()
        .await
        .unwrap();

    loser
        .call(&worker, contract.id(), "make_bid")
        .args_json(&args_for_bid)
        .unwrap()
        .deposit(parse_near!("1 N"))
        .transact()
        .await
        .unwrap();

    winner
        .call(&worker, contract.id(), "make_bid")
        .args_json(&args_for_bid)
        .unwrap()
        .deposit(parse_near!("5 N"))
        .transact()
        .await
        .unwrap();

    owner
        .call(&worker, contract.id(), "produce_auction")
        .transact()
        .await
        .unwrap();

    let winner_items: Vector<String> = winner
        .call(&worker, contract.id(), "get_items")
        .transact()
        .await
        .unwrap()
        .borsh()
        .unwrap();

    assert_eq!(
        winner_items.len(),
        2,
        "Incorrect items amount. Should be 2, actual {}",
        winner_items.len()
    );

    let seller_acc = seller.view_account(&worker).await.unwrap();

    assert_eq!(
        yocto_to_token(seller_acc.balance).ceil(),
        35f64,
        "Seller has invalid amount of money. Should be 35 N, actual: {}",
        yocto_to_token(seller_acc.balance).ceil()
    );

    assert_eq!(
        yocto_to_token(loser_acc.balance).ceil(),
        20f64,
        "Loser has invalid amount of money. Should be 20 N, actual: {}",
        yocto_to_token(loser_acc.balance).ceil()
    );
}

#[tokio::test]
async fn test_auction_with_two_items() {
    let worker = workspaces::sandbox().await.unwrap();
    let wasm = std::fs::read(WASM_FILEPATH).unwrap();
    let contract = worker.dev_deploy(&wasm).await.unwrap();

    let owner = worker.root_account();

    owner
        .call(&worker, contract.id(), "new")
        .transact()
        .await
        .unwrap();

    owner
        .call(&worker, contract.id(), "start_new_auction")
        .transact()
        .await
        .unwrap();

    let winner = owner
        .create_subaccount(&worker, WINNER_ACC_ID)
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await
        .unwrap()
        .unwrap();

    let seller_1 = owner
        .create_subaccount(&worker, SELLER_ACC_ID)
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await
        .unwrap()
        .unwrap();

    let seller_2 = owner
        .create_subaccount(&worker, format!("{}_1", SELLER_ACC_ID).as_str())
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await
        .unwrap()
        .unwrap();

    let loser = owner
        .create_subaccount(&worker, LOSER_ACC_ID)
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await
        .unwrap()
        .unwrap();

    let args_for_sell_1: Value = from_str(
        r#"
        {
            "item":"test_item",
            "min_bid": 0
        }"#,
    )
    .unwrap();

    let args_for_bid_1: Value = from_str(
        r#"{
                "item_hash":"68E5EE009D13B901BBB36D3BB47FC59ACA581D6DB141DA0574287495244A9225"
            }
            "#,
    )
    .unwrap();

    let args_for_sell_2: Value = from_str(
        r#"
                {
                    "item":"another_test_item",
                "min_bid": 0
            }"#,
    )
    .unwrap();

    let args_for_bid_2: Value = from_str(
        r#"{
                    "item_hash":"AD2AFDA91E9D009272A01459110D14D0AAD7F4648412CE04B2B5E5F322DC527E"
                }
                "#,
    )
    .unwrap();

    seller_1
        .call(&worker, contract.id(), "add_item_to_auction")
        .args_json(args_for_sell_1.clone())
        .unwrap()
        .transact()
        .await
        .unwrap();

    seller_2
        .call(&worker, contract.id(), "add_item_to_auction")
        .args_json(args_for_sell_2.clone())
        .unwrap()
        .transact()
        .await
        .unwrap();

    loser
        .call(&worker, contract.id(), "make_bid")
        .args_json(&args_for_bid_1)
        .unwrap()
        .deposit(parse_near!("5 N"))
        .transact()
        .await
        .unwrap();

    winner
        .call(&worker, contract.id(), "make_bid")
        .args_json(&args_for_bid_1)
        .unwrap()
        .deposit(parse_near!("10 N"))
        .transact()
        .await
        .unwrap();

    loser
        .call(&worker, contract.id(), "make_bid")
        .args_json(&args_for_bid_2)
        .unwrap()
        .deposit(parse_near!("5 N"))
        .transact()
        .await
        .unwrap();

    winner
        .call(&worker, contract.id(), "make_bid")
        .args_json(&args_for_bid_2)
        .unwrap()
        .deposit(parse_near!("10 N"))
        .transact()
        .await
        .unwrap();

    owner
        .call(&worker, contract.id(), "produce_auction")
        .transact()
        .await
        .unwrap();

    let winner_items: Vector<String> = winner
        .call(&worker, contract.id(), "get_items")
        .transact()
        .await
        .unwrap()
        .borsh()
        .unwrap();

    assert_eq!(
        winner_items.len(),
        2,
        "Incorrect items amount. Should be 2, actual {}",
        winner_items.len()
    );

    let seller_acc_1 = seller_1.view_account(&worker).await.unwrap();
    let seller_acc_2 = seller_2.view_account(&worker).await.unwrap();
    let loser_acc = loser.view_account(&worker).await.unwrap();

    assert_eq!(
        yocto_to_token(seller_acc_1.balance).ceil(),
        40f64,
        "Seller has invalid amount of money. Should be 40 N, actual: {}",
        yocto_to_token(seller_acc_1.balance).ceil(),
    );

    assert_eq!(
        yocto_to_token(seller_acc_2.balance).ceil(),
        40f64,
        "Seller has invalid amount of money. Should be 40 N, actual: {}",
        yocto_to_token(seller_acc_2.balance).ceil(),
    );

    assert_eq!(
        yocto_to_token(loser_acc.balance).ceil(),
        30f64,
        "Seller has invalid amount of money. Should be 30 N, actual: {}",
        yocto_to_token(loser_acc.balance).ceil(),
    );
}

#[tokio::test]
#[should_panic]
async fn bid_less_than_min_bid() {
    let worker = workspaces::sandbox().await.unwrap();
    let wasm = std::fs::read(WASM_FILEPATH).unwrap();
    let contract = worker.dev_deploy(&wasm).await.unwrap();

    let owner = worker.root_account();

    owner
        .call(&worker, contract.id(), "new")
        .transact()
        .await
        .unwrap();

    owner
        .call(&worker, contract.id(), "start_new_auction")
        .transact()
        .await
        .unwrap();

    let bidder = owner
        .create_subaccount(&worker, WINNER_ACC_ID)
        .initial_balance(parse_near!("20 N"))
        .transact()
        .await
        .unwrap()
        .unwrap();

    let seller = owner
        .create_subaccount(&worker, SELLER_ACC_ID)
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await
        .unwrap()
        .unwrap();

    let args_for_sell: Value = from_str(
        r#"
    {
        "item":"test_item",
        "min_bid": 2
    }"#,
    )
    .unwrap();

    let args_for_bid: Value = from_str(
        r#"{
            "item_hash":"68E5EE009D13B901BBB36D3BB47FC59ACA581D6DB141DA0574287495244A9225"
    }
        "#,
    )
    .unwrap();

    seller
        .call(&worker, contract.id(), "add_item_to_auction")
        .args_json(args_for_sell.clone())
        .unwrap()
        .transact()
        .await
        .unwrap();

    bidder
        .call(&worker, contract.id(), "make_bid")
        .args_json(args_for_bid)
        .unwrap()
        .deposit(1u128)
        .transact()
        .await
        .unwrap();
}



#[tokio::test]
#[should_panic]
async fn bid_to_non_exists_item() {
    let worker = workspaces::sandbox().await.unwrap();
    let wasm = std::fs::read(WASM_FILEPATH).unwrap();
    let contract = worker.dev_deploy(&wasm).await.unwrap();

    let owner = worker.root_account();

    owner
        .call(&worker, contract.id(), "new")
        .transact()
        .await
        .unwrap();

    owner
        .call(&worker, contract.id(), "start_new_auction")
        .transact()
        .await
        .unwrap();

    let bidder = owner
        .create_subaccount(&worker, WINNER_ACC_ID)
        .initial_balance(parse_near!("20 N"))
        .transact()
        .await
        .unwrap()
        .unwrap();

    let args_for_bid: Value = from_str(
        r#"{
            "item_hash":"68E5EE009D13B901BBB36D3BB47FC59ACA581D6DB141DA0574287495244A9225"
    }
        "#,
    )
    .unwrap();

    bidder
        .call(&worker, contract.id(), "make_bid")
        .args_json(args_for_bid)
        .unwrap()
        .deposit(1u128)
        .transact()
        .await
        .unwrap();
}
