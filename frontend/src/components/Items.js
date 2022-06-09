import React from 'react'
import './Items.css'

function Items() {
    const items = [1, 2, 3, 4, 5, 6, 7, 8, 1, 1, 1, 1, 1, 1, 1]
    return (
        <div style={{ width: '100%', height: '100%' }}>
            <ul class="tilesWrap">
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