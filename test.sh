#!/bin/bash
# near call contract.msolomodenko.testnet --accountId=contract.msolomodenko.testnet new
near call contract.msolomodenko.testnet --accountId=contract.msolomodenko.testnet start_new_auction
near call contract.msolomodenko.testnet --accountId=seller.msolomodenko.testnet add_test_item
near call contract.msolomodenko.testnet --deposit 1 --accountId=looser.msolomodenko.testnet make_bid '{"item_hash":"68E5EE009D13B901BBB36D3BB47FC59ACA581D6DB141DA0574287495244A9225"}'
near call contract.msolomodenko.testnet --deposit 2 --accountId=msolomodenko.testnet make_bid '{"item_hash":"68E5EE009D13B901BBB36D3BB47FC59ACA581D6DB141DA0574287495244A9225"}'
near call contract.msolomodenko.testnet --accountId=contract.msolomodenko.testnet produce_auction
