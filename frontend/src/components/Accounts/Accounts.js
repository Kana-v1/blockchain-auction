import React from 'react'
import './Accounts.css'
import { login, getAccountId, logout } from '../../contract/utils'

function newActiveAccount(accounts) {
    let IDs = Array.from(accounts.keys())
    accounts = new Map()

    IDs.map((value) => {
        return accounts.set(value, false)
    })

    console.log(JSON.stringify(Array.from(accounts.entries())))

    localStorage.setItem('usersAccounts', JSON.stringify(Array.from(accounts.entries())))

    logout()
    login()
}

function Accounts() {
    let existingAccs = JSON.parse(localStorage.getItem('usersAccounts'))
    let accs = new Map()

    for (let i = 0; i < existingAccs?.length; i++) {
        accs.set(existingAccs[i][0], existingAccs[i][1])
    }

    const accId = getAccountId()
    if (accId !== "") {
        accs.delete(accId)
        accs.set(accId, true)
    }

    const accounts = Array.from(accs.keys())

    return (
        <div className='accounts' style={{ borderColor: "green" }}>
            <ul className='accList'>
                {accounts.map((value, key) => {
                    return (
                        <li key={key} className='accRow' onClick={() => {
                            newActiveAccount(accs)
                        }}>
                            {value}
                            {accs.get(value) === true ? <div className="activeAccSign"></div> : ''}
                        </li>
                    )
                })}

                <button className="newAcc" onClick={() => {
                    newActiveAccount(accs)
                }}>Add new account</button>
            </ul>
        </div>)
}

export default Accounts
