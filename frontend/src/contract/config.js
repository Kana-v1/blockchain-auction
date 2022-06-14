const CONTRACT_NAME = 'contract.msolomodenko.testnet'

function getConfig() {
    return {
        networkId: 'testnet',
        nodeUrl: 'https://rpc.testnet.near.org',
        contractName: CONTRACT_NAME,
        walletUrl: 'https://wallet.testnet.near.org',
        helperUrl: 'https://helper.testnet.near.org'
    }
}

export default getConfig;