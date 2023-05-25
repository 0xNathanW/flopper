import { useState } from "react";
import Board from "./Board";
import RangeSelector from "./range/RangeSelector";
import TreeConfig from "./tree/TreeConfig";
import "./Config.css";
import BetSizes from "./tree/BetSizes";

export type TreeConfigValues = {
    startingPot:    number,
    effectiveStack: number,
    rake:           number,
    rakeCap:        number,
    addAllIn:       number,
    forceAllIn:     number,
}

export type WeightsProps = {
    weightsIP: number[],
    weightsOOP: number[],
    setWeightsIP: (weights: number[]) => void,
    setWeightsOOP: (weights: number[]) => void,
}

export default function Config() {

    // Weights array.
    const [weightsOOP, setWeightsOOP] = useState<number[]>(Array(169).fill(0));
    const [weightsIP, setWeightsIP]   = useState<number[]>(Array(169).fill(0));

    // Idxs of the board cards.
    const [board, setBoard] = useState<number[]>([]);

    // Tree config.
    const [treeConfig, setTreeConfig] = useState<TreeConfigValues>({
        startingPot:    40,
        effectiveStack: 100,
        rake:           0,
        rakeCap:        3,
        addAllIn:       0,
        forceAllIn:     0,
    });

    return (
        <div id="config">
            <div id="config-left"> 
                <RangeSelector 
                    weightsIP={weightsIP}
                    weightsOOP={weightsOOP}
                    setWeightsIP={setWeightsIP}
                    setWeightsOOP={setWeightsOOP}
                />
            </div>
        
            <div id="config-right">
                <Board board={board} setBoard={setBoard}/> 
                <BetSizes />
                <TreeConfig treeConfig={treeConfig} setTreeConfig={setTreeConfig} />
            </div>
        </div>
    )
}