import Board from "./Board";
import RangeSelector from "./range/RangeSelector";
import TreeConfig from "./tree/TreeConfig";
import "./Config.css";
import BetSizes from "./tree/BetSizes";

export default function Config() {
    return (
        <div id="config">
            <div id="config-left"> 
                <RangeSelector />
            </div>
        
            <div id="config-right">
                    <Board /> 
                <BetSizes />
                <TreeConfig />
                <Build />
            </div>
        </div>
    )
}

function Build() {

    return (
        <div id="build">
            <button className="build-button">Build Tree</button>
            <button className="build-button">Preview Tree</button>
            <button className="build-button">Run Solver</button>
        </div>
    )
}