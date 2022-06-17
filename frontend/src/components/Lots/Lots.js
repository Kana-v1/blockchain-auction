import React from 'react'
import './Lots.css'
import UpdateIcon from '@mui/icons-material/Update';
import { getLots, makeBid } from '../../contract/utils'
import * as nearAPI from "near-api-js";
const { utils } = nearAPI;

export default function Auctions() {
    let [lots, setLots] = React.useState([])
    let [updateColor, setUpdateColor] = React.useState('green')
    let [buttonIsAble, setButtonAble] = React.useState(true)

    React.useEffect(() => {
        getLots().then(result => setLots(result))
    }, [])

    return (
        <div style={{ width: '100%', height: '100%' }}>


            <ul className='auctions'>
                <div style={{ color: updateColor, position: 'relative', left: '1%', cursor: 'pointer' }} onClick={() => {
                    if (updateColor === 'red') {
                        return
                    }

                    setUpdateColor('red')
                    getLots().then(lots => setLots(lots))
                    setUpdateColor('green')
                }}>
                    <UpdateIcon />
                </div>
                <div className  = "text">
                    {lots.length === 0 ? 'There are no active lots now' : ''}
                </div>

                {lots.map((value, key) => {
                    if (value.are_u_supplier) {
                        return null
                    }

                    return (
                        <li key={key}>
                            <div className="itemEl">{value.item}</div>
                            <hr className="titleLine" />
                            <div className="itemEl">
                                <div className="con-tooltip right winnerHelper" style={{ borderColor: value.are_u_winner ? "green" : "red" }}
                                >
                                    {utils.format.formatNearAmount(value.current_bid)}
                                    <div className="tooltip"><p>{value.are_u_winner ? "your bid is winning" : "your bid is loosing"}</p></div>
                                </div>
                                <button onClick={() => {
                                    setButtonAble(false)

                                    let attachedDeposit = prompt("Please enter amount of the tokens you want to bid", "1")
                                    if (attachedDeposit === null || attachedDeposit === "" || attachedDeposit === undefined) {
                                        attachedDeposit = "1"
                                    }


                                    makeBid(value.item_hash, utils.format.parseNearAmount(attachedDeposit))
                                    getLots().then(lots => setLots(lots))

                                    setButtonAble(true)
                                }} disabled={!buttonIsAble} style={{ backgroundColor: buttonIsAble ? '#24245c' : 'grey', cursor: buttonIsAble ? 'pointer' : '' }}>Make a bid</button>
                            </div>
                        </li>
                    )
                })}
            </ul>
        </div >
    )
}