import Board from "./Board";
import RangeSelector from "./range/RangeSelector";
import TreeConfig from "./tree/TreeConfig";
import BetSizes from "./tree/BetSizes";
import { useState } from "react";
import "./Config.css";
import BuildTree from "./BuildTree";

export default function Config() {
    return (
        <div id="config">

            <div id="config-left">
                <RangeSelector />
            </div>
        
            <div id="config-right">
                <div id="config-right-upper">
                    <Board /> 
                    <TreeConfig />
                </div>
                <BetSizes />
                <BuildTree />
            </div>
        </div>
    )
}
