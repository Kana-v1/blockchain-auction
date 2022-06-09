import React from 'react'
import { SidebarData } from './SidebarData'
import ArrowForwardIosIcon from '@mui/icons-material/ArrowForwardIos';
import MenuIcon from '@mui/icons-material/Menu';

function Sidebar(props) {

    const [fullList, setFullList] = React.useState(false)

    return (
        <div className={fullList ? 'sidebar' : 'smallSidebar'} >
            <ul className='sidebarList'>
                <div className={!fullList ? 'menuRow' : 'cutMenu'} onClick={() => setFullList(!fullList)}>
                    <MenuIcon/>
                </div>
                {SidebarData.map((value, key) => {
                    return (
                        <li key={key} className='row' onClick={() => {
                            if (value.title === 'Account' && !fullList) {
                                return
                            }

                            props.clicked.has(value.title) ? props.clicked.delete(value.title) : props.clicked.add(value.title)
                            props.setClicked(new Set(props.clicked))
                        }}
                            id={props.clicked.has(value.title) ? 'active' : ''}>
                            <div className="line"></div>
                            <div id='icon'>{value.icon}</div>

                            {fullList ? <div id="title">{value.title}</div> : ''}

                            {value.title === 'Account' && fullList ? <ArrowForwardIosIcon id={props.clicked.has(value.title) ? 'account-arrow-active' : 'account-arrow'} /> : ''}
                        </li>
                    )
                })}
            </ul>
        </div>
    )
}

export default Sidebar