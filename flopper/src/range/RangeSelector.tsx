import { useState } from "react"
import "./RangeSelector.css"
import RangeGrid from "./RangeGrid"
import WeightInput from "./WeightSelect";
import LoadRange from "./LoadRange";

export default function RangeSelector() {
    // Weight to be applied to selected cells.
    const [weight, setWeight] = useState(100);
    // Wheter we are using the OOP or IP range.
    const [oop, setOOP] = useState(true);
    // Weights array.
    const [weightsOOP, setWeightsOOP] = useState<number[]>(Array(169).fill(0));
    const [weightsIP, setWeightsIP] = useState<number[]>(Array(169).fill(0));
    
    return (
        <div id="config-left-panel">
            <div id="range-selector">
                <div id="range-header">
                    <PlayerToggle />
                    <h1>Range</h1>
                </div>
                    <RangeGrid   weight={weight} weights={weightsOOP} setWeights={setWeightsOOP} />
                    <WeightInput setWeight={setWeight} weight={weight} setWeights={setWeightsOOP} />
            </div>
            <LoadRange setWeights={setWeightsOOP} />
        </div>
    )
}

function PlayerToggle() {

    return (
        <label className="toggle">
            <input className="toggle-input" type="checkbox" />
            <span className="toggle-handle"></span>
            <span className="toggle-label" data-on="IP" data-off="OOP"></span>
        </label>
    )
}

