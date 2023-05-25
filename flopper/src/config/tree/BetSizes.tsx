import "./BetSizes.css";

export default function BetSizes() {

    return (
        <div id="bet-sizes">
            <BetSizesStreetPlayer oop={true}/>
            <BetSizesStreetPlayer oop={false}/>
        </div>
    )
}

function BetSizesStreetPlayer({oop}: {oop: boolean}) {

    return (
        <>
            <h2>Bet Sizes: {oop ? "OOP" : "IP"}</h2>
            <div className="bet-sizes-player">
                <BetSizesStreet street={0} oop={oop} />
                <BetSizesStreet street={1} oop={oop} />
                <BetSizesStreet street={2} oop={oop} />
            </div>
        </>
    )
}

function BetSizesStreet({ street, oop }: { street: number, oop: boolean }) {
    
    return (
        <fieldset className="bet-sizes-street">
            <legend>{streetText(street)}</legend>   
            <div id="bet-sizes-grid">
                <label>Bet:</label> 
                <input type="text" className="bet-input"></input>
            
                <label>Raise:</label>
                <input type="text" className="bet-input"></input>
            
                <label>All-in</label>
                <input type="checkbox" className="all-in"></input>
            </div>
        </fieldset>
    )
}

function streetText(street: number) {
    if (street === 0) {
        return "Flop";
    } else if (street === 1) {
        return "Turn";
    } else {
        return "River";
    }
}