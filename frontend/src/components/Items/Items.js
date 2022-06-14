import React from 'react'
import './Items.css'

function Items() {
    const items = [1, 2, 3, 4, 5, 6, 7, 8, 11, 12, 13, 15, 14, 17, 51]
    return (
        <div style={{ width: '100%', height: '100%' }}>
            <ul className="tilesWrap">
                {items.map((key, value) => {
                    return (
                        <li key={key}>
                            <h2>{key}</h2>
                            <p>Some item</p>
                        </li>)
                })}
            </ul>
        </div>
    )
}

export default Items