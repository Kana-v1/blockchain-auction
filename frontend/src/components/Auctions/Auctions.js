import React from 'react'
import './Auctions.css'

function Auctions({ children }) {
    const auctions = [
        {
            is_owner: true,
            item: "some_item",
            current_bid: 1,
            are_u_winner: false,
        },
        {
            is_owner: false,
            item: "some_item_1",
            current_bid: 1,
            are_u_winner: true,
        },
        {
            is_owner: false,
            item: "another_item",
            current_bid: 2,
            are_u_winner: false,
        },
        {
            is_owner: false,
            item: "another_item",
            current_bid: 2,
            are_u_winner: false,
        },
    ]

    return (
        <div style={{ width: '100%', height: '100%' }}>
            <ul className='auctions'>
                {auctions.map((value, key) => {
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