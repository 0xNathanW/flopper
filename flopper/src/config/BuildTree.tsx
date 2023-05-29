import { useState } from "react";
import { Config } from "../store/features/configSlice";
import { useAppSelector } from "../store/store";
import "./BuildTree.css";
import { invoke } from "@tauri-apps/api";

export default function BuildTree() {

    const config = useAppSelector(state => state.config);
    const [error, setError] = useState("");

    const handleClick = () => {
        const errs = verifyConfig(config);
        if (errs.length > 0) {
            setError(errs[0]);
        } else {
            invoke("build_action_tree", {
                board: config.board,
                startingPot: config.startingPot,
                effectiveStack: config.effectiveStack,
                rake: config.rake,
                rakeCap: config.rakeCap,
                addAllInThreshold: config.addAllIn,
                forceAllInThreshold: config.forceAllIn,
                
                oopBetsFlop: config.betSizes[0][0][0],
                oopRaisesFlop: config.betSizes[0][0][1],

                oopBetsTurn: config.betSizes[0][1][0],
                oopRaisesTurn: config.betSizes[0][1][1],

                oopBetsRiver: config.betSizes[0][2][0],
                oopRaisesRiver: config.betSizes[0][2][1],

                ipBetsFlop: config.betSizes[1][0][0],
                ipRaisesFlop: config.betSizes[1][0][1],

                ipBetsTurn: config.betSizes[1][1][0],
                ipRaisesTurn: config.betSizes[1][1][1],

                ipBetsRiver: config.betSizes[1][2][0],
                ipRaisesRiver: config.betSizes[1][2][1],
            });

            setError("");
        }
    }

    const showError = () => {
        if (error !== "") {
            return <p className="build-tree-error">{error}</p>
        }
    }

    return (
        <div id="build-tree">
            <button 
                className="build-tree-btn"
                onClick={() => {handleClick()}}
            >Build Action Tree</button>
            {showError()}
        </div>
    )
}

function verifyConfig(config: Config) {

    const errors = [];

    // Check that board is valid.
    if (config.board.length < 3) {
        errors.push("Board must have at least 3 cards.");
    }

    // Check if any empty bet sizes.
    config.betSizes.forEach((player, p) => {
        const playerTxt = p === 0 ? "IP" : "OOP";
        player.forEach((street, s) => {
            const streetTxt = s === 0 ? "Flop" : s === 1 ? "Turn" : "River";
            street.forEach((raise, r) => {
                const raiseTxt = r === 0 ? "Bet" : "Raise";
                if (raise === "") {
                    errors.push(`${playerTxt} ${streetTxt} ${raiseTxt} is empty.`);
                }
            });
        });
    });

    // Check ranges are not empty.
    if (config.rangeIP.reduce((a, b) => a + b) === 0) {
        errors.push("IP range is empty.");
    }
    if (config.rangeOOP.reduce((a, b) => a + b) === 0) {
        errors.push("OOP range is empty.");
    }
    
    return errors;
}
    