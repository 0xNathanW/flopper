import { useEffect, useState } from "react";
import { useAppDispatch, useAppSelector } from "../store/store";
import { 
    ConfigTree,
    copyOOP, 
    setAddAllIn, 
    setBetSize, 
    setEffectiveStack, 
    setForceAllIn, 
    setRake, 
    setRakeCap, 
    setStartingPot 
} from "../store/features/configSlice";

const syntaxTip = `suffixes: 'x' for scaled bet, 'c' for constant bet, '%' for percentage bet, 'e' for geometric bet, 'allin' for all-in bet.`

export default function TreeConfig() {

    const [triggerCopy, setTriggerCopy] = useState(false);

    const config = useAppSelector(state => state.config);
    const dispatch = useAppDispatch();

    const handleCopyClick = () => {
        dispatch(copyOOP());
        setTriggerCopy(true);
        setTimeout(() => {
            setTriggerCopy(false);
        });
    }

    return (
        <div className="flex flex-col items-center">
            <div className="flex flex-col items-center w-fit-content gap-5">
                <div className="flex flex-row items-center gap-3 self-start">
                    <h1 className="text-2xl self-start font-bold">OOP Bet Sizes:</h1>
                    <div className="tooltip tooltip-right tooltip-info" data-tip={syntaxTip}>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
                            <path strokeLinecap="round" strokeLinejoin="round" d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9 5.25h.008v.008H12v-.008z" />
                        </svg>
                    </div>
                </div>
                <div className="flex flex-row items-center gap-5 mt-3">
                    <div className="indicator">
                        <span className="indicator-item indicator-center badge badge-secondary badge-lg z-0">Flop</span>
                        <StreetBetSize oop={true} street={0} trigger={false} />
                    </div>
                    <div className="indicator">
                        <span className="indicator-item indicator-center badge badge-secondary badge-lg z-0">Turn</span>
                        <StreetBetSize oop={true} street={1} trigger={false} />
                    </div>
                    <div className="indicator">
                        <span className="indicator-item indicator-center badge badge-secondary badge-lg z-0">River</span>
                        <StreetBetSize oop={true} street={2} trigger={false} />
                    </div>
                </div>
                
                <div className="divider my-0"></div>
                <div className="flex flex-row justify-between items-center w-full">
                    <h1 className="text-2xl self-start font-bold">IP Bet Sizes:</h1>
                    <button className="btn btn-primary btn-sm" onClick={handleCopyClick}>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-5 h-5">
                            <path strokeLinecap="round" strokeLinejoin="round" d="M15.75 17.25v3.375c0 .621-.504 1.125-1.125 1.125h-9.75a1.125 1.125 0 01-1.125-1.125V7.875c0-.621.504-1.125 1.125-1.125H6.75a9.06 9.06 0 011.5.124m7.5 10.376h3.375c.621 0 1.125-.504 1.125-1.125V11.25c0-4.46-3.243-8.161-7.5-8.876a9.06 9.06 0 00-1.5-.124H9.375c-.621 0-1.125.504-1.125 1.125v3.5m7.5 10.375H9.375a1.125 1.125 0 01-1.125-1.125v-9.25m12 6.625v-1.875a3.375 3.375 0 00-3.375-3.375h-1.5a1.125 1.125 0 01-1.125-1.125v-1.5a3.375 3.375 0 00-3.375-3.375H9.75" />
                        </svg>
                        Copy OOP
                    </button>
                </div>
                <div className="flex flex-row items-center gap-5 mt-3">
                    <div className="indicator">
                        <span className="indicator-item indicator-center badge badge-secondary badge-lg z-0">Flop</span>
                        <StreetBetSize oop={false} street={0} trigger={triggerCopy} />
                    </div>
                    <div className="indicator">
                        <span className="indicator-item indicator-center badge badge-secondary badge-lg z-0">Turn</span>
                        <StreetBetSize oop={false} street={1} trigger={triggerCopy} />
                    </div>
                    <div className="indicator">
                        <span className="indicator-item indicator-center badge badge-secondary badge-lg z-0">River</span>
                        <StreetBetSize oop={false} street={2} trigger={triggerCopy} />
                    </div>
                </div>
                
                <div className="divider my-0"></div>
                <div className="grid grid-cols-[7rem,5rem,7rem,5rem] items-center gap-10">
                    
                    <p>Starting Pot:</p>
                    <input
                        type="number"
                        min={1}
                        value={config.startingPot}
                        onChange={(e) => dispatch(setStartingPot(Number(e.target.value)))}
                        className="input input-bordered input-secondary input-sm w-full"
                    ></input>

                    <p>Effective Stack:</p>
                    <input
                        type="number"
                        min={1}
                        value={config.effectiveStack}
                        onChange={(e) => dispatch(setEffectiveStack(Number(e.target.value)))}
                        className="input input-bordered input-secondary input-sm w-full"
                    ></input>

                    <p>Rake %:</p>
                    <input
                        type="number"
                        min={0}
                        max={100}
                        step={0.5}
                        value={config.rake}
                        onChange={(e) => dispatch(setRake(Number(e.target.value)))}
                        className="input input-bordered input-secondary input-sm w-full"
                    ></input>

                    <p>Rake Cap:</p>
                    <input
                        type="number"
                        min={0}
                        value={config.rakeCap}
                        onChange={(e) => dispatch(setRakeCap(Number(e.target.value)))}
                        className="input input-bordered input-secondary input-sm w-full"
                    ></input>

                    <p>Add All-In %:</p>
                    <input
                        type="number"
                        min={0}
                        step={5}
                        value={config.addAllIn}
                        onChange={(e) => dispatch(setAddAllIn(Number(e.target.value)))}
                        className="input input-bordered input-secondary input-sm w-full"
                    ></input>

                    <p>Force All-In %:</p>
                    <input
                        type = "number"
                        min={0}
                        step={5}
                        value={config.forceAllIn}
                        onChange={(e) => dispatch(setForceAllIn(Number(e.target.value)))}
                        className="input input-bordered input-secondary input-sm w-full"
                    ></input>
                </div>

                {/* <div className="divider my-0"></div>
                <div className="flex flex-row items-start align-middle w-full">
                    <button className="btn btn-primary" onClick={() => {
                    }}
                    >Build Action Tree</button>
                </div> */}
            </div>
        </div>
    )
}

function StreetBetSize({oop, street, trigger}: {oop: boolean, street: number, trigger: boolean}) {
    
    const dispatch = useAppDispatch();
    const betSizes = useAppSelector(state => state.config.betSizes);

    const [betValue, setBetValue] = useState(betSizes[+oop][street][0]);
    const [raiseValue, setRaiseValue] = useState(betSizes[+oop][street][1]);

    const [betError, setBetError] = useState("");
    const [raiseError, setRaiseError] = useState("");

    useEffect(() => {
        if (trigger && !oop) {
            const oopBet = betSizes[0][street][0];
            const oopRaise = betSizes[0][street][1];

            if (oopBet !== "error" && oopBet !== "") {
                setBetValue(oopBet);
                dispatch(setBetSize([oop, street, false, oopBet]));
            }
            if (oopRaise !== "error" && oopRaise !== "") {
                setRaiseValue(oopRaise);
                dispatch(setBetSize([oop, street, true, oopRaise]));
            }
        }
    }, [trigger]);

    const handleInput = (e: React.FormEvent<HTMLInputElement>, raise: boolean) => {
        e.preventDefault();
        const err = verifyBetTxt(e.currentTarget.value, false);
        if (err.length > 0) {
            raise ? setRaiseError(err[0]) : setBetError(err[0]);
            dispatch(setBetSize([oop, street, raise, "error"]));
            raise ? setRaiseValue(e.currentTarget.value) : setBetValue(e.currentTarget.value);
        } else {
            raise ? setRaiseError("") : setBetError("");
            dispatch(setBetSize([oop, street, raise, e.currentTarget.value]));
            raise ? setRaiseValue(e.currentTarget.value) : setBetValue(e.currentTarget.value);
        }
    }

    const badge = (raise: boolean) => {
        if (raise ? raiseError !== "" : betError !== "") {
            return (
                <div className="tooltip tooltip-error" data-tip={raise ? raiseError : betError}>
                    <div className="badge badge-error badge-sm">Invalid</div>
                </div>
            )
        } else if (raise ? raiseValue === "" : betValue === "") {
            return <div className="badge badge-warning badge-sm">Empty</div>
        } else {
            return <div className="badge badge-success badge-sm">Valid</div>
        }
    }

    return (
        <div className="grid grid-cols-[3rem,10rem,3rem] gap-3 p-4 pt-5 border-2 border-secondary rounded-lg items-center">
            <label>Bet:</label>
            <input 
                type="text" 
                className="input input-bordered input-secondary input-sm w-full bg-"
                value={betValue}
                onInput={(e) => handleInput(e, false)}
            ></input>
            {badge(false)}

            <label>Raise:</label>
            <input
                type="text"
                className="input input-bordered input-secondary input-sm w-full"
                value={raiseValue}
                onInput={(e) => handleInput(e, true)}
            ></input>
            {badge(true)}
        </div>
    )
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