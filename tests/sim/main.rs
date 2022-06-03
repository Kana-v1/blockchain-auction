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

    owner.call(&worker, contract.id(), "new");
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
        .call(&worker, contract.id(), "get_all_items_vec")
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
        .call(&worker, contract.id(), "get_all_items_vec")
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
async fn test() {
    let mut io = UnorderedMap::<i32, i32>::new(b"m");
    io.insert(&2, &3);
    io.insert(&1, &3);
    println!("{}",io.get(&1).unwrap());

    println!("{:?}", io);
}

#[tokio::test]
#[allow(unused_variables)]
async fn two_auctions_in_sequence() {
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
        .call(&worker, contract.id(), "get_all_items_vec")
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
      /* #endregion*/

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


    let logs = owner
        .call(&worker, contract.id(), "produce_auction")
        .transact()
        .await
        .unwrap()
        .logs();
        

    let winner_items: Vector<String> = winner
        .call(&worker, contract.id(), "get_all_items_vec")
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
        40f64,
        "Seller has invalid amount of money. Should be 40 N, actual: {}",
        seller_acc.balance
    );

    assert_eq!(
        yocto_to_token(loser_acc.balance).ceil(),
        20f64,
        "Seller has invalid amount of money. Should be 20 N, actual: {}",
        loser_acc.balance
    );
}
