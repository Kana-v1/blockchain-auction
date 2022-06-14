import React from 'react'
import SellIcon from '@mui/icons-material/Sell';
import DataArrayIcon from '@mui/icons-material/DataArray';
// import ShoppingCartIcon from '@mui/icons-material/ShoppingCart';
import AccountCircleIcon from '@mui/icons-material/AccountCircle';

export const SidebarData = [
    {
        title: 'Accounts',
        icon: <AccountCircleIcon />,
    },
    {
        title: 'Auctions',
        icon: <SellIcon />,
    },
    {
        title: 'Items',
        icon: <DataArrayIcon />,
    }
]


