import "./WeightSelect.css";

type WeightSelectProps = {
    weight: number,
    setWeight: (weight: number) => void,
    setWeights: (weights: number[]) => void,
}

export default function WeightSelect(props: WeightSelectProps) {
    
    const handleWeightChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        props.setWeight(Number(e.target.value));
    }

    return (
        <div id="weight-input-div">
            <p>Weight:</p>
            <input 
                id="weight-slider" 
                type="range" 
                min="0"
                max="100" 
                value={props.weight}
                onChange={handleWeightChange}
            />
            <input
                id="weight-input"
                type="number"
                min="0"
                max="100"
                step="5"
                value={props.weight}
                onChange={handleWeightChange}
            />
            <p> %</p>

            <button id="clear" onClick={() => props.setWeights(Array(169).fill(0))}>Clear</button>
        </div>
    )
}
