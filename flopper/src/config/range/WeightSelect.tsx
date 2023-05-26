import { useAppDispatch } from "../../store/store";
import "./WeightSelect.css";
import { clearRangeIP, clearRangeOOP } from "../../store/features/configSlice";

type WeightSelectProps = {
    oop: boolean,
    weight: number,
    setWeight: (weight: number) => void,
}

export default function WeightSelect(props: WeightSelectProps) {

    const dispatch = useAppDispatch();

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

            <button id="clear" onClick={() => {
                if (props.oop) {
                    dispatch(clearRangeOOP());
                } else {
                    dispatch(clearRangeIP());
                }
            }}>Clear</button>
        </div>
    )
}
