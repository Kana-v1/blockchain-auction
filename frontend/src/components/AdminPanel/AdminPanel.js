import './AdminPanel.css'
import React from 'react'
import { addItemToAuction, isAuctionOpen, produceAuction, startNewAuction } from '../../contract/utils'
import * as nearAPI from "near-api-js";
const { utils } = nearAPI;


function updateAuctionState(changeAuctionState) {
    isAuctionOpen().then(isAuctionOpen => {
        changeAuctionState(isAuctionOpen)
    })
}

export default function AdminPanel() {
    const [auctionCreated, changeAuctionState] = React.useState(false)
    const [item, setItem] = React.useState('')
    const [itemMinBid, setItemMinBid] = React.useState('')

    const [addItemBtnDisabled, setAddItemBtnState] = React.useState(true)
    const [createNewAucBrnDisabled, setNewAucBtnState] = React.useState(true)
    const [produceAucBtnDisabled, setProduceAucBtnState] = React.useState(true)

    React.useEffect(() => {
        isAuctionOpen().then(isAuctionOpen => {
            changeAuctionState(isAuctionOpen)
            setAddItemBtnState(false)
            setNewAucBtnState(false)
            setProduceAucBtnState(false)
        })

        const interval = setInterval(() => {
            updateAuctionState(changeAuctionState)
        }, 5000)
        return () => clearInterval(interval)
    }, [])

    return (
        <div className="adminPanel">
            <div className='addNewItem'>
                <div className='groupWrapper'>
                    <div className="group">
                        <input type="text" required
                            value={item}
                            onChange={(e) => setItem(e.target.value)} />
                        <span className="highlight"></span>
                        <span className="bar"></span>
                        <label>Item</label>
                    </div>
                    <div className="group">
                        <input type="text" required
                            value={itemMinBid}
                            onChange={(e) => {
                                e.target.value = e.target.value.replace(/\D/g, '') // remove non-numeric values

                                if (e.target.value !== '') {
                                    setItemMinBid(Number(e.target.value))
                                } else {
                                    setItemMinBid('')
                                }
                            }}
                        />
                        <span className="highlight"></span>
                        <span className="bar"></span>
                        <label>Minimal bid (N) &gt;=1</label>
                    </div>
                </div>
                <button disabled={!auctionCreated || addItemBtnDisabled} className={auctionCreated && !addItemBtnDisabled ? 'newItemBtn' : 'newItemBtn btnDisabled'} onClick={() => {
                    if (item === '' || itemMinBid === '' || itemMinBid < 1) {
                        return
                    }

                    setAddItemBtnState(true)

                    addItemToAuction(item, utils.format.parseNearAmount(itemMinBid.toString()))
                        .then(() => {
                            setItemMinBid('');
                            setItem('')
                            setAddItemBtnState(false)
                        })
                }}>{addItemBtnDisabled ? '...' : 'Add new item'}</button>
            </div>
            <div className='auctionActions'>
                <button disabled={auctionCreated} className={auctionCreated || createNewAucBrnDisabled ? 'btnDisabled' : ''} onClick={() => {
                    setNewAucBtnState(true)

                    startNewAuction().then(() => {
                        updateAuctionState(changeAuctionState)
                        setNewAucBtnState(false)
                    })
                }}>{createNewAucBrnDisabled ? '...' : 'Create new auction'}</button>
                <button disabled={!auctionCreated} className={auctionCreated && !produceAucBtnDisabled ? '' : 'btnDisabled'} onClick={() => {
                    setProduceAucBtnState(true)

                    produceAuction().then(() => {
                        updateAuctionState(changeAuctionState)
                        setProduceAucBtnState(false)
                    })
                }}>{produceAucBtnDisabled ? '...' : 'Produce an auction'}</button>
            </div>
        </div>
    )
}