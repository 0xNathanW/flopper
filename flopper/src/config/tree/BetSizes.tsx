import { useEffect, useState } from "react";
import "./BetSizes.css";
import { useAppDispatch, useAppSelector } from "../../store/store";
import { setBetSize, copyOOP } from "../../store/features/configSlice";

export default function BetSizes() {

    return (
        <div id="bet-sizes">
            <BetSizesStreetOOP />
            <BetSizesStreetIP />
        </div>
    )
}

function BetSizesStreetOOP() {
    return (
        <>
            <div className="bet-sizes-header">
                <h2>Bet Sizes: OOP</h2>
                <div className="tooltip">?
                    <span className="tooltiptext">
                        <h3>Bet Size Syntax:</h3>
                        <p>Constant Bet Size: 1c, 20c, 300c, ...</p>
                        <p>Scaled Previous Bet (raise only): 2x, 2.5x, 5x, ...</p>
                        <p>Percentage of Pot: 33%, 50%, 120%, ...</p>
                        <p>Geometric Bet (num streets [e] pot limit): 1e2, 1e3, 2e2, ...</p>
                        <p>All-in: allin, a</p>
                    </span>
                </div>
            </div>
            <div className="bet-sizes-player">
                <BetSizesStreet street={0} oop={true} trigger={false} />
                <BetSizesStreet street={1} oop={true} trigger={false} />
                <BetSizesStreet street={2} oop={true} trigger={false} />
            </div>
        </>
    )
}

function BetSizesStreetIP() {

    const dispatch = useAppDispatch();

    const [triggerCopy, setTriggerCopy] = useState(false);

    // This is so bad, but whatever :P
    const handleClick = () => {
        dispatch(copyOOP()); // copy bet sizes in store.
        setTriggerCopy(true);
        setTimeout(() => {
            setTriggerCopy(false);
        });
    }

    return (
        <>
            <div className="bet-sizes-header">
                <h2>Bet Sizes: IP</h2>
                <button 
                    className="copy-btn"
                    onClick={() => handleClick()}    
                >Copy OOP</button>
            </div>
            <div className="bet-sizes-player">
                <BetSizesStreet street={0} oop={false} trigger={triggerCopy} />
                <BetSizesStreet street={1} oop={false} trigger={triggerCopy} />
                <BetSizesStreet street={2} oop={false} trigger={triggerCopy} />
            </div>
        </>
    )
}

function BetSizesStreet({ street, oop, trigger }: { street: number, oop: boolean, trigger: boolean }) {
    
    const [betError, setBetError] = useState("");
    const [raiseError, setRaiseError] = useState("");

    const [betVal, setBetVal] = useState("");
    const [raiseVal, setRaiseVal] = useState("");

    const dispatch = useAppDispatch();
    const betSizes = useAppSelector((state) => state.config.betSizes);

    useEffect(() => {
        if (trigger && !oop) {
            setBetVal(betSizes[1][street][0]);
            setRaiseVal(betSizes[1][street][1]);
            dispatch(setBetSize([oop, street, false, betSizes[1][street][0]]));
            dispatch(setBetSize([oop, street, true, betSizes[1][street][1]]));
        }
    }, [trigger]);

    const onChange = (e: React.ChangeEvent<HTMLInputElement>, raise: boolean) => {
        e.preventDefault();
        const errors = verifyBetTxt(e.target.value, raise);
        if (errors.length > 0) { // There is an error
            raise ? setRaiseError(errors[0]) : setBetError(errors[0]);
            dispatch(setBetSize([oop, street, raise, ""])); // set bet size to empty string
            raise ? setRaiseVal(e.target.value) : setBetVal(e.target.value);
        } else {
            raise ? setRaiseError("") : setBetError("");
            dispatch(setBetSize([oop, street, raise, e.target.value]));
            raise ? setRaiseVal(e.target.value) : setBetVal(e.target.value);
        }
    }

    const showBetError = () => {
        if (betError !== "") {
            return <p className="bet-error">{betError}</p>
        }
    }

    const showRaiseError = () => {
        if (raiseError !== "") {
            return <p className="bet-error">{raiseError}</p>
        }
    }

    return (
        <fieldset className="bet-sizes-street">
            <legend>{streetText(street)}</legend>   
            <div id="bet-sizes-grid">
                <label>Bet:</label> 
                <input 
                    type="text" 
                    className="bet-input"
                    id={`bet-input-${oop ? "OOP": "IP"}-${street}`}
                    onChange={(e) => onChange(e, false)}
                    value={betVal}
                ></input>

                <label>Raise:</label>
                <input 
                    type="text" 
                    className="bet-input"
                    id={`raise-input-${oop ? "OOP": "IP"}-${street}`}
                    onChange={(e) => onChange(e, true)}
                    value={raiseVal}
                ></input>

            </div>
            {showBetError()}
            {showRaiseError()}
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

function verifyBetTxt(s: string, raise: boolean) {

    const betTxt = raise ? "Raise" : "Bet";

    let errors: string[] = [];

    const trimmed = s.split(",").map((bet) => {
        return bet.trim().toLowerCase();
    }).filter((bet) => {
        return !(bet === "")
    });

    trimmed.forEach((bet) => {

        if (bet === "allin" || bet === "a") {

            
        } else if (bet.includes("e")) {
            const split = bet.split("e");
            const num_streets = Number(split[0]);
            const max_pot = Number(split[1]);
            if (!(split[0] === "")) {
                if (isNaN(num_streets) || num_streets < 1 || num_streets > 100 || !Number.isInteger(num_streets)) {
                    errors.push("Geometric Bet: Number of streets must be an integer between 1 and 100");
                }
            }
            if (!(split[1] === "")) {
                if (isNaN(max_pot)) {
                    errors.push("Geometric Bet: Maximum pot limit must be a number");
                }
            }
        } else {
            switch (bet[bet.length - 1]) {
                case "x":
                    if (!raise) {
                        errors.push("Scaled Bet: Can only use 'x' for raises");
                    }
                    const betN = Number(bet.slice(0, -1));
                    if (isNaN(betN) || betN < 0) {
                        errors.push("Scaled Bet: Must be a positive integer");
                    }
                    break;
                
                case "c":
                    const cN = Number(bet.slice(0, -1));
                    if (isNaN(cN) || cN < 0) {
                        errors.push("Constant Bet: Must be a positive integer");
                    }
                    break;
                
                case "%": 
                    const pctN = Number(bet.slice(0, -1));
                    if (isNaN(pctN) || pctN < 0) {
                        errors.push("Percentage Bet: Must be a positive integer");
                    }
                    break;
                
                default:
                    errors.push(`Invalid ${betTxt}: Must end in 'x', 'c', or '%' or be 'allin'/'a'`);
            }
        }
    });

    return errors;
}