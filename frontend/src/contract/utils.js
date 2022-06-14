import { connect, Contract, WalletConnection, keyStores } from 'near-api-js'
import getConfig from './config'

const nearConfig = getConfig()

export async function initContract() {
    const near = await connect(Object.assign({
        deps: {
            keyStore: new keyStores.BrowserLocalStorageKeyStore()
        }
    }, nearConfig))

    window.walletConnection = new WalletConnection(near)
    
    window.accountId = window.walletConnection.getAccountId()

    window.contract = new Contract(window.walletConnection.account(), nearConfig.contractName, {
        viewMethods: ['get_items'],
        changeMethods: ['add_item_to_auction', 'produce_auction', 'make_bid', 'start_new_auction']
    })
}

export function logout() {
    window.walletConnection.signOut()
}

export function login() {
    window.walletConnection.requestSignIn({ contractId: nearConfig.contractName })
}

export async function getItems() {
    let items = await window.contract.get_items({ args: {} }).catch(err => errorHandler(err))
    return items
}

export async function addItemToAuction(item, minBid) {
    await window.contract.add_item_to_auction({ args: { item: item, min_bid: minBid } }).catch(err => errorHandler(err))
}

export async function produceAuction() {
    await window.contract.produce_auction({ args: {} }).catch(err => errorHandler(err))
}

export async function makeBid(itemHash) {
    await window.contract.make_bid({ args: { item_hash: itemHash } }).catch(err => errorHandler(err))
}

export async function startNewAuction() {
    await window.contract.start_new_auction({ args: {} }).catch(err => errorHandler(err))
}

export function getAccountId() {
    return window.walletConnection.getAccountId()
}


function errorHandler(err) {
    console.log(err)
}
