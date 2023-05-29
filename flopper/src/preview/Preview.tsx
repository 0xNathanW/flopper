import { useState } from "react";
import { ActionNode } from "../common";
import "./Preview.css";
import { useAppSelector } from "../store/store";

export default function Preview() {

    const config = useAppSelector(state => state.config);

    const [nodes, setNodes] = useState<ActionNode[]>([
        {
            type: "root",
            idx: 0,
            player: config.board.length === 3 ? "flop" : config.board.length === 4 ? "turn" : "river",
            selectedIdx: -1,
            board: config.board,
            pot: config.startingPot,
            stack: config.effectiveStack,
        }
    ])
    const [idx, setIdx] = useState(-1);

    return (
        <div id="preview">
            <div id="preview-tree">
            </div>
        </div>
    )
}

function RootChanceNode({node}: {node: ActionNode}) {

    return (
        <div className="root-chance">
            <h3>Start</h3>
        </div>
    )
}