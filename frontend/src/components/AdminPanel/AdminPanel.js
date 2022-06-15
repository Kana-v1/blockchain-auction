import './AdminPanel.css'

export default function AdminPanel() {
    return (
        <div className="adminPanel">
            <div className='addNewItem'>
                <div className='groupWrapper'>
                <div class="group">
                    <input type="text" required/>
                        <span class="highlight"></span>
                        <span class="bar"></span>
                        <label>Item</label>
                </div>
                <div class="group">
                    <input type="text" required/>
                        <span class="highlight"></span>
                        <span class="bar"></span>
                        <label>Minimal bid</label>
                </div>
                </div>
                <button class = "newItemBtn">Add Item</button>
            </div>
            <div className='auctionActions'>
                <button>Create new auction</button>
                <button>Produce the auction</button>
            </div>
        </div>
    )
}