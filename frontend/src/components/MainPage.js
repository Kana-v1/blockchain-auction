import React from 'react'
import Sidebar from './Sidebar'
import Bids from './Bids'
import Items from './Items'
import Auctions from './Auctions'
import './MainPage.css'

function MainPage() {
    const [clicked, setClicked] = React.useState(new Set())
    return (
        <div style={{ height: '100%', display: 'flex' }}>
            <Sidebar style={{ innerHeight: '100%' }} clicked={clicked} setClicked={setClicked} />
            <div style={{ height: '100%', width: '100%', display: 'flex', flexDirection: 'column' }}>
                <div style={{ height: '100%', width: '100%', display: 'flex' }}>
                    <div className="bids" style={clicked.has('Bids') ? { visibility: 'visible' } : { visibility: 'hidden'}}><Bids /></div>
                    <div className="items"style={clicked.has('Items') ? { visibility: 'visible' } : { visibility: 'hidden'}}><Items /></div>
                </div>
                <div className="auctions" style={clicked.has('Auctions') ? { visibility: 'visible' } : { visibility: 'hidden'}}><Auctions /></div>
            </div>
        </div>
    )
}

export default MainPage
