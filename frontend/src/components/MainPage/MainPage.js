import React from 'react'
import Sidebar from '../Sidebar/Sidebar'
import Items from '../Items/Items'
import Lots from '../Lots/Lots'
import Accounts from '../Accounts/Accounts'
import AdminPanel from '../AdminPanel/AdminPanel'
import './MainPage.css'


function MainPage() {
    const [clicked, setClicked] = React.useState(new Set())

    return (
        <div style={{ height: '100%', display: 'flex' }}>
            <Sidebar style={{ innerHeight: '100%' }}
                clicked={clicked}
                setClicked={setClicked} />
            <div className="technicalWrapper">
                <div style={clicked.has('Account') ? { visibility: 'visible' } : { visibility: 'hidden' }}>
                    <Accounts />
                </div>

                <div style={clicked.has('Admin panel') ? { visibility: 'visible' } : { visibility: 'hidden' }}>
                    <AdminPanel />
                </div>
            </div>

            <div style={{ height: '100%', width: '100%' }}>
                <div style={{ height: '100%', width: '100%', display: 'flex' }}>
                    <div style={clicked.has('Lots') ? { visibility: 'visible', width: '50%' } : { visibility: 'hidden', width: '50%' }}><Lots /></div>
                    <div className="items sides border" style={clicked.has('Items') ? { visibility: 'visible' } : { visibility: 'hidden' }}><Items /></div>
                </div>
            </div>

        </div>
    )
}

export default MainPage
