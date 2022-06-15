import React from 'react'
import './Items.css'
import UpdateIcon from '@mui/icons-material/Update';
import {getItems} from '../../contract/utils'


function Items() {
    const items = [1, 2, 3, 4, 5, 6, 7, 8, 11, 12, 13, 15, 14, 17, 51]
    let [updateColor, setUpdateColor] = React.useState('green')
    return (
        <div style={{ width: '100%', height: '100%' }}>
            <div style= {{color: updateColor, position: 'absolute', right: '10px', zIndex: '1', cursor: 'pointer'}} onClick = {() => {
                if (updateColor === 'red') {
                    return 
                }

                setUpdateColor('red')
                let items = getItems()
                console.log(items)
                setUpdateColor('green')

            }}>
            <UpdateIcon />
            </div>
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