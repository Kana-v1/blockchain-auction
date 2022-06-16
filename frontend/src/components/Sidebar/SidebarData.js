import React from 'react'
import SellIcon from '@mui/icons-material/Sell';
import DataArrayIcon from '@mui/icons-material/DataArray';
import AccountCircleIcon from '@mui/icons-material/AccountCircle';
import AdminPanelSettingsIcon from '@mui/icons-material/AdminPanelSettings';

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
    },
    {
        title: 'Admin panel',
        icon: <AdminPanelSettingsIcon />,
    }
]


