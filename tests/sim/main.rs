use near_units::parse_near;
use serde_json::json;
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
async fn test_single_participant() -> anyhow::Result<()> {
    /* #region  init*/
    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(WASM_FILEPATH)?;
    let contract = worker.dev_deploy(&wasm).await?;

    let owner = worker.root_account();

    owner.call(&worker, contract.id(), "new").transact().await?;

    owner
        .call(&worker, contract.id(), "start_new_auction")
        .transact()
        .await?;

    let winner = owner
        .create_subaccount(&worker, WINNER_ACC_ID)
        .initial_balance(parse_near!("20 N"))
        .transact()
        .await?
        .into_result()?;

    let seller = owner
        .create_subaccount(&worker, SELLER_ACC_ID)
        .initial_balance(parse_near!("20 N"))
        .transact()
        .await?
        .into_result()?;
    /* #endregion*/

    let args_for_sell = json!(
        {
            "item":"test_item",
            "min_bid": "0"
        }
    );

    seller
        .call(&worker, contract.id(), "add_item_to_auction")
        .args_json(args_for_sell)?
        .transact()
        .await?;

    let args_for_bid: Value = json!(
        {
            "item_hash":"68E5EE009D13B901BBB36D3BB47FC59ACA581D6DB141DA0574287495244A9225"
        }
    );

    winner
        .call(&worker, contract.id(), "make_bid")
        .args_json(args_for_bid)?
        .deposit(parse_near!("10 N"))
        .transact()
        .await?;

    owner
        .call(&worker, contract.id(), "produce_auction")
        .transact()
        .await?;

    let get_items_args = json!({ "account_id": format!("{}.test.near", WINNER_ACC_ID) });

    let winner_items: Vec<String> = winner
        .call(&worker, contract.id(), "get_items")
        .args_json(get_items_args)?
        .transact()
        .await?
        .json()?;

    assert_eq!(
        winner_items.len(),
        1,
        "Incorrect items amount. Should be 1, actual {}",
        winner_items.len()
    );

    let acc = seller.view_account(&worker).await?;

    assert_eq!(
        yocto_to_token(acc.balance).ceil(), // much less than 1 token has been burned as gas
        30f64,
        "Seller has invalid amount of money. Should be 20 N, actual: {}",
        acc.balance
    );

    Ok(())
}

#[tokio::test]
async fn test_multiple_participants() -> anyhow::Result<()> {
    /* #region  init*/
    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(WASM_FILEPATH)?;
    let contract = worker.dev_deploy(&wasm).await?;

    let owner = worker.root_account();

    owner.call(&worker, contract.id(), "new").transact().await?;

    owner
        .call(&worker, contract.id(), "start_new_auction")
        .transact()
        .await?;

    let winner = owner
        .create_subaccount(&worker, WINNER_ACC_ID)
        .initial_balance(parse_near!("20 N"))
        .transact()
        .await?
        .into_result()?;

    let seller = owner
        .create_subaccount(&worker, SELLER_ACC_ID)
        .initial_balance(parse_near!("20 N"))
        .transact()
        .await?
        .into_result()?;

    let loser = owner
        .create_subaccount(&worker, LOSER_ACC_ID)
        .initial_balance(parse_near!("20 N"))
        .transact()
        .await?
        .into_result()?;

    let args_for_bid = json!(
        {
            "item_hash":"68E5EE009D13B901BBB36D3BB47FC59ACA581D6DB141DA0574287495244A9225"
        }
    );

    let args_for_sell = json!(
        {
            "item":"test_item",
            "min_bid": "0"
        }
    );
    /* #endregion*/

    seller
        .call(&worker, contract.id(), "add_item_to_auction")
        .args_json(args_for_sell)?
        .transact()
        .await?;

    loser
        .call(&worker, contract.id(), "make_bid")
        .args_json(&args_for_bid)?
        .deposit(parse_near!("5 N"))
        .transact()
        .await?;

    winner
        .call(&worker, contract.id(), "make_bid")
        .args_json(&args_for_bid)?
        .deposit(parse_near!("10 N"))
        .transact()
        .await?;

    owner
        .call(&worker, contract.id(), "produce_auction")
        .transact()
        .await?;

    let get_items_args = json!({ "account_id": format!("{}.test.near", WINNER_ACC_ID) });

    let winner_items: Vec<String> = winner
        .call(&worker, contract.id(), "get_items")
        .args_json(get_items_args)?
        .transact()
        .await?
        .json()?;

    assert_eq!(
        winner_items.len(),
        1,
        "Incorrect items amount. Should be 1, actual {}",
        winner_items.len()
    );

    let seller_acc = seller.view_account(&worker).await?;

    assert_eq!(
        yocto_to_token(seller_acc.balance).ceil(),
        30f64,
        "Seller has invalid amount of money. Should be 20 N, actual: {}",
        seller_acc.balance
    );

    let loser_acc = loser.view_account(&worker).await?;

    assert_eq!(
        yocto_to_token(loser_acc.balance).ceil(),
        20f64,
        "Seller has invalid amount of money. Should be 30 N, actual: {}",
        loser_acc.balance
    );

    Ok(())
}

#[tokio::test]
async fn two_two_auctions_in_sequence() -> anyhow::Result<()> {
    /* #region  init*/
    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(WASM_FILEPATH)?;
    let contract = worker.dev_deploy(&wasm).await?;

    let owner = worker.root_account();

    owner.call(&worker, contract.id(), "new").transact().await?;

    owner
        .call(&worker, contract.id(), "start_new_auction")
        .transact()
        .await?;

    let winner = owner
        .create_subaccount(&worker, WINNER_ACC_ID)
        .initial_balance(parse_near!("20 N"))
        .transact()
        .await?
        .into_result()?;

    let seller = owner
        .create_subaccount(&worker, SELLER_ACC_ID)
        .initial_balance(parse_near!("20 N"))
        .transact()
        .await?
        .into_result()?;

    let loser = owner
        .create_subaccount(&worker, LOSER_ACC_ID)
        .initial_balance(parse_near!("20 N"))
        .transact()
        .await?
        .into_result()?;

    let args_for_bid = json!(
        {
            "item_hash":"68E5EE009D13B901BBB36D3BB47FC59ACA581D6DB141DA0574287495244A9225"
        }
    );

    let args_for_sell = json!(
        {
            "item":"test_item",
            "min_bid": "0"
        }
    );
    /* #endregion*/
    seller
        .call(&worker, contract.id(), "add_item_to_auction")
        .args_json(args_for_sell.clone())?
        .transact()
        .await?;

    loser
        .call(&worker, contract.id(), "make_bid")
        .args_json(&args_for_bid)?
        .deposit(parse_near!("5 N"))
        .transact()
        .await?;

    winner
        .call(&worker, contract.id(), "make_bid")
        .args_json(&args_for_bid)?
        .deposit(parse_near!("10 N"))
        .transact()
        .await?;

    owner
        .call(&worker, contract.id(), "produce_auction")
        .transact()
        .await?;

    let get_items_args = json!({ "account_id": format!("{}.test.near", WINNER_ACC_ID) });

    let winner_items: Vec<String> = winner
        .call(&worker, contract.id(), "get_items")
        .args_json(get_items_args)?
        .transact()
        .await?
        .json()?;

    assert_eq!(
        winner_items.len(),
        1,
        "Incorrect items amount. Should be 1, actual {}",
        winner_items.len()
    );

    let seller_acc = seller.view_account(&worker).await?;
    let loser_acc = loser.view_account(&worker).await?;

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
        .await?;

    seller
        .call(&worker, contract.id(), "add_item_to_auction")
        .args_json(args_for_sell)?
        .transact()
        .await?;

    loser
        .call(&worker, contract.id(), "make_bid")
        .args_json(&args_for_bid)?
        .deposit(parse_near!("1 N"))
        .transact()
        .await?;

    winner
        .call(&worker, contract.id(), "make_bid")
        .args_json(&args_for_bid)?
        .deposit(parse_near!("5 N"))
        .transact()
        .await?;

    owner
        .call(&worker, contract.id(), "produce_auction")
        .transact()
        .await?;

    let get_items_args = json!({ "account_id": format!("{}.test.near", WINNER_ACC_ID) });

    let winner_items: Vec<String> = winner
        .call(&worker, contract.id(), "get_items")
        .args_json(get_items_args)?
        .transact()
        .await?
        .json()?;

    assert_eq!(
        winner_items.len(),
        2,
        "Incorrect items amount. Should be 2, actual {}",
        winner_items.len()
    );

    let seller_acc = seller.view_account(&worker).await?;

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

    Ok(())
}

#[tokio::test]
async fn test_auction_with_two_items() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(WASM_FILEPATH)?;
    let contract = worker.dev_deploy(&wasm).await?;

    let owner = worker.root_account();

    owner.call(&worker, contract.id(), "new").transact().await?;

    owner
        .call(&worker, contract.id(), "start_new_auction")
        .transact()
        .await?;

    let winner = owner
        .create_subaccount(&worker, WINNER_ACC_ID)
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    let seller_1 = owner
        .create_subaccount(&worker, SELLER_ACC_ID)
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    let seller_2 = owner
        .create_subaccount(&worker, format!("{}_1", SELLER_ACC_ID).as_str())
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    let loser = owner
        .create_subaccount(&worker, LOSER_ACC_ID)
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    let args_for_sell_1 = json!(
        {
            "item":"test_item",
            "min_bid": "0"
        }
    );

    let args_for_bid_1 = json!(
        {
                "item_hash":"68E5EE009D13B901BBB36D3BB47FC59ACA581D6DB141DA0574287495244A9225"
            }
    );

    let args_for_sell_2 = json!(
        {
            "item":"another_test_item",
            "min_bid": "0"
        }
    );

    let args_for_bid_2 = json!(
        {
            "item_hash":"AD2AFDA91E9D009272A01459110D14D0AAD7F4648412CE04B2B5E5F322DC527E"
        }
    );

    seller_1
        .call(&worker, contract.id(), "add_item_to_auction")
        .args_json(args_for_sell_1.clone())?
        .transact()
        .await?;

    seller_2
        .call(&worker, contract.id(), "add_item_to_auction")
        .args_json(args_for_sell_2.clone())?
        .transact()
        .await?;

    loser
        .call(&worker, contract.id(), "make_bid")
        .args_json(&args_for_bid_1)?
        .deposit(parse_near!("5 N"))
        .transact()
        .await?;

    winner
        .call(&worker, contract.id(), "make_bid")
        .args_json(&args_for_bid_1)?
        .deposit(parse_near!("10 N"))
        .transact()
        .await?;

    loser
        .call(&worker, contract.id(), "make_bid")
        .args_json(&args_for_bid_2)?
        .deposit(parse_near!("5 N"))
        .transact()
        .await?;

    winner
        .call(&worker, contract.id(), "make_bid")
        .args_json(&args_for_bid_2)?
        .deposit(parse_near!("10 N"))
        .transact()
        .await?;

    owner
        .call(&worker, contract.id(), "produce_auction")
        .transact()
        .await?;

    let get_items_args = json!({ "account_id": format!("{}.test.near", WINNER_ACC_ID) });

    let winner_items: Vec<String> = winner
        .call(&worker, contract.id(), "get_items")
        .args_json(get_items_args)?
        .transact()
        .await?
        .json()?;

    assert_eq!(
        winner_items.len(),
        2,
        "Incorrect items amount. Should be 2, actual {}",
        winner_items.len()
    );

    let seller_acc_1 = seller_1.view_account(&worker).await?;
    let seller_acc_2 = seller_2.view_account(&worker).await?;
    let loser_acc = loser.view_account(&worker).await?;

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

    Ok(())
}

#[tokio::test]
#[should_panic(expected="This item has 2 minimum bid. Actual: 1")]
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

    let args_for_sell = json!(
        {
            "item":"test_item",
            "min_bid": "2"
        }
    );

    let args_for_bid = json!(
        {
            "item_hash":"68E5EE009D13B901BBB36D3BB47FC59ACA581D6DB141DA0574287495244A9225"
        }
    );

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
#[should_panic(expected="Item with hash 68E5EE009D13B901BBB36D3BB47FC59ACA581D6DB141DA0574287495244A9225 does not exist")]
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

    let args_for_bid = json!(
        {
            "item_hash":"68E5EE009D13B901BBB36D3BB47FC59ACA581D6DB141DA0574287495244A9225"
        }
    );

    bidder
        .call(&worker, contract.id(), "make_bid")
        .args_json(args_for_bid)
        .unwrap()
        .deposit(1u128)
        .transact()
        .await
        .unwrap();
}
