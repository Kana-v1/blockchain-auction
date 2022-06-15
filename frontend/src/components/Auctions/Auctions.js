import React from 'react'
import './Auctions.css'
import UpdateIcon from '@mui/icons-material/Update';
import {getLots} from '../../contract/utils'



function Auctions() {
    let [lots, setLots] = React.useState([])
    let [updateColor, setUpdateColor] = React.useState('green')

    getLots().then(lots => setLots(lots))

    return (
        <div style={{ width: '100%', height: '100%' }}>
                        <div style= {{color: updateColor, position: 'absolute', left: '70px', zIndex: '1', cursor: 'pointer'}} onClick = {() => {
                if (updateColor === 'red') {
                    return 
                }

                setUpdateColor('red')
                getLots().then(lots => setLots(lots))
                setUpdateColor('green')

            }}>
            <UpdateIcon />
            </div>

            <ul className='auctions'>
                {lots.map((value, key) => {
                    return (
                        <li key={key}>
                            <div className="itemEl">{value.item}</div>
                            <hr className="titleLine" />
                            <div className="itemEl">
                                <div className="con-tooltip right winnerHelper" style={{ borderColor: value.are_u_winner ? "green" : "red" }}
                                >
                                    {value.current_bid}
                                    <div className="tooltip"><p>{value.are_u_winner ? "your bid is winning" : "your bid is loosing"}</p></div>
                                </div>
                            </div>
                            <button>Make bid</button>
                        </li>
                    )
                })}
            </ul>
        </div>
    )
}

export default Auctions