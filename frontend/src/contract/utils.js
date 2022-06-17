import { connect, Contract, WalletConnection, keyStores, DEFAULT_FUNCTION_CALL_GAS } from 'near-api-js'
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
        viewMethods: ['get_lots', 'get_items', 'get_auction_state'],
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
    return await window.contract.get_items({ account_id: getAccountId().toString() }).catch(err => errorHandler(err))
}


export async function isAuctionOpen() {
    return await window.contract.get_auction_state({}).catch(err => errorHandler(err))
}

export async function getLots() {
    let lots = await window.contract.get_lots({ args: {} }).catch(err => errorHandler(err))
    if (lots === null || lots === undefined || lots === '') {
        return
    }

    try {
        lots = JSON.parse(lots)
        let suitableLots = []
        lots.forEach(lot => {
            suitableLots.push({
                is_owner: true,
                item: lot.item,
                current_bid: lot.current_bid.toLocaleString('fullwide', { useGrouping: false }),
                are_u_winner: getAccountId() === lot.winner,
                are_u_supplier: getAccountId() === lot.supplier,
                item_hash: lot.item_hash
            })
        })

        return suitableLots

    }
    catch (err) {
        errorHandler(err)
    }
}

export async function addItemToAuction(item, minBid) {
    console.log('addin')
    await window.contract.add_item_to_auction({ args: { item: item, min_bid: minBid } }).catch(err => errorHandler(err))
}

export async function produceAuction() {
    await window.contract.produce_auction({ args: {} }).catch(err => errorHandler(err))
}

export async function makeBid(itemHash, attachedDeposit) {
    await window.contract.make_bid({ item_hash: itemHash }, DEFAULT_FUNCTION_CALL_GAS, attachedDeposit).catch(err => errorHandler(err))
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
