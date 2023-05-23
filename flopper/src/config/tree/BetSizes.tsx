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
        <div className="bet-sizes-player">
            <BetSizesStreet street={0} />
            <BetSizesStreet street={1} />
            <BetSizesStreet street={2} />
        </div>
    )
}

function BetSizesStreet({ street }: { street: number }) {
    return (
        <div className="bet-sizes-street">
            <h3>{streetText(street)}</h3>       
            <div className="bet-grid">
                <p>Bet:</p> 
                <input type="text" className="bet-input"></input>
                <p>Raise:</p>
                <input type="text" className="bet-input"></input>
            </div>
        </div>
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