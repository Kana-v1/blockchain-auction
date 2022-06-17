import React from 'react'
import './Items.css'
import UpdateIcon from '@mui/icons-material/Update';
import { getItems } from '../../contract/utils'


function Items() {
    let [items, setItems] = React.useState([])

    let [updateColor, setUpdateColor] = React.useState('red')

    React.useEffect(() => {
        getItems().then(result => {
            if (result !== undefined && result !== null) {
                setItems(result)
            }
            
            setUpdateColor('green')
        })
    }, [])

    return (
        <div style={{ width: '100%', height: '100%' }}>
            <div style={{ color: updateColor, position: 'absolute', right: '10px', zIndex: '1', cursor: 'pointer' }} onClick={() => {
                if (updateColor === 'red') {
                    return
                }

                setUpdateColor('red')
                getItems().then(items => {
                    setItems(items)
                    setUpdateColor('green')
                })

            }}>
                <UpdateIcon />
            </div>
            <ul className="tilesWrap">
                {items.map((value, key) => {
                    return (
                        <li key={key}>
                            <h2>{key + 1}</h2>
                            <p>{value}</p>
                        </li>)
                })}
            </ul>
        </div>
    )
}

export default Items