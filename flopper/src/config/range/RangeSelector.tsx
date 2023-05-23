import { useState } from "react"
import "./RangeSelector.css"
import RangeGrid from "./RangeGrid"
import WeightInput from "./WeightSelect";
import { WeightsProps } from "../Config";
import LoadRange from "./LoadRange";

export default function RangeSelector(props: WeightsProps) {
    // Weight to be applied to selected cells.
    const [weight, setWeight] = useState(100);
    // Wheter we are using the OOP or IP range.
    const [oop, setOOP] = useState(false);
    
    return (
        <>
            <div id="range-selector">
                <div id="range-header">
                    <PlayerToggle setOOP={setOOP} oop={oop}/>
                    <h1>Range</h1>
                </div>
                    <RangeGrid
                        weight={weight} 
                        weights={ oop ? props.weightsOOP : props.weightsIP } 
                        setWeights={ oop ? props.setWeightsOOP : props.setWeightsIP } 
                    />
                    <WeightInput 
                        setWeight={setWeight} 
                        weight={weight} 
                        setWeights={ oop ? props.setWeightsOOP : props.setWeightsIP } 
                    />
            <LoadRange setWeights={ oop ? props.setWeightsOOP : props.setWeightsIP } />
            </div>
        </>
    )
}

type PlayerToggleProps = {
    setOOP: (oop: boolean) => void,
    oop: boolean,
}

function PlayerToggle(props: PlayerToggleProps) {

    const handleClick = () => {
        props.setOOP(!props.oop);
        console.log(props.oop);
    }

    return (
        <label className="toggle">
            <input className="toggle-input" type="checkbox" onClick={handleClick}/>
            <span className="toggle-handle"></span>
            <span className="toggle-label" data-on="IP" data-off="OOP"></span>
        </label>
    )
}

