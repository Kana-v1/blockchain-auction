use near_sdk::collections::Vector;
use near_units::parse_near;
use serde_json::from_str;
use serde_json::Value;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Sender;
use workspaces::prelude::*;
use workspaces::{network::Sandbox, Account, Contract, Worker};

use std::{thread, time};

const SELLER_ACC_ID: &str = "seller";
const WINNER_ACC_ID: &str = "winner";
const LOSER_ACC_ID: &str = "loser";
const WASM_FILEPATH: &str = "./res/auction.wasm";

fn yocto_to_token(n: u128) -> f64 {
    n as f64 / 10u128.pow(24) as f64
}

struct TestSetup {
    supplier: Account,
    winner: Account,
    loser: Account,
    owner: Account,
    worker: Worker<Sandbox>,
    contract: Contract,
}

impl TestSetup {
    fn new(
        supplier: Account,
        winner: Account,
        loser: Account,
        owner: Account,
        worker: Worker<Sandbox>,
        contract: Contract,
    ) -> Self {
        Self {
            supplier,
            winner,
            loser,
            owner,
            worker,
            contract,
        }
    }
}

#[tokio::main]
async fn init(p: Sender<TestSetup>) {
    let worker = workspaces::sandbox().await.unwrap();
    let wasm = std::fs::read(WASM_FILEPATH).unwrap();
    let contract = worker.dev_deploy(&wasm).await.unwrap();

    let owner = worker.root_account();

    let winner = owner
        .create_subaccount(&worker, WINNER_ACC_ID)
        .initial_balance(parse_near!("10 N"))
        .transact()
        .await
        .unwrap()
        .unwrap();

    let supplier = owner
        .create_subaccount(&worker, SELLER_ACC_ID)
        .initial_balance(parse_near!("10 N"))
        .transact()
        .await
        .unwrap()
        .unwrap();

    let loser = owner
        .create_subaccount(&worker, LOSER_ACC_ID)
        .initial_balance(parse_near!("10 N"))
        .transact()
        .await
        .unwrap()
        .unwrap();

    owner.call(&worker, contract.id(), "new");

    if let Err(_) = p.send(TestSetup::new(
        supplier, winner, loser, owner, worker, contract,
    )) {
        panic!("can not send TestSetup")
    }
}

#[tokio::test]
async fn test_single_participant() {
    let (p, mut c) = oneshot::channel::<TestSetup>();
    
    init(p);

    let mut setup: Vec<TestSetup> = Vec::new();

    loop {
        match c.try_recv() {
            Ok(msg) => {
                setup.push(msg);
                break;
            }
            Err(_) => thread::sleep(time::Duration::from_millis(2000)),
        }
    }

    let worker = &setup[0].worker;
    let seller = &setup[0].supplier;
    let owner = &setup[0].owner;
    let contract = &setup[0].contract;
    let winner = &setup[0].winner;

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
        yocto_to_token(acc.balance).ceil(), // much less 1 token has been burned as gas
        30f64,
        "Seller has invalid amount of money. Should be 20 N, actual: {}",
        acc.balance
    );
}
