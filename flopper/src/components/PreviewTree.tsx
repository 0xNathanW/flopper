import { useEffect, useState } from "react";
import { ActionNode, ChanceNode, PlayerNode, RootNode, TerminalNode } from "../node_types"
import { useAppSelector } from "../store/store";
import * as rust from "../rust_funcs";

type SelectData = {
    nodes: ActionNode[];
    selectedIdx: number;
    bet: number;
    totalBet: number[];
    prevBet: number;
    locked: boolean;
};

export default function PreviewTree() {
    
    const [trigger, setTrigger] = useState(false);
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
    ]);
    const [selectedNodeIdx, setSelectedNodeIdx] = useState(-1);
    const [betAmount, setBetAmount] = useState(0);
    const [totalBetAmount, setTotalBetAmount] = useState([0, 0]);
    const [prevBetAmount, setPrevBetAmount] = useState(0);
    const [locked, setLocked] = useState(false);
    
    const isAfterAllIn = () => {
        const maxTotalBetAmount = Math.max(...totalBetAmount);
        return maxTotalBetAmount >= config.effectiveStack;
    };
    
    const isSelectedTerminal = () => {
        if (locked || selectedNodeIdx === -1) {
            return false;
        }
        const selectedNode = nodes[selectedNodeIdx];
        return selectedNode.type === "terminal";
    };


    const maxAmount = () => {
        if (isSelectedTerminal()) return 0;
        const maxTotalBetAmount = Math.max(...totalBetAmount);
        return config.effectiveStack - (maxTotalBetAmount - prevBetAmount);
    };

    const minAmount = () => {
        const betMinus = config.effectiveStack - maxAmount();
        const min = Math.min(...totalBetAmount) - betMinus;
        const max = Math.max(...totalBetAmount) - betMinus;
        return Math.min(Math.max(2 * max - min, 1), maxAmount());
    };

    const amountRate = () => {
        const pot = config.startingPot + 2 * Math.max(...totalBetAmount);
        const amount = betAmount + prevBetAmount;
        return amount / pot;
    };

    const existingAmount = () => {
        if (locked || selectedNodeIdx === -1) return [];
        const out = [];
        const node = nodes[selectedNodeIdx] as PlayerNode;
        for (const action of node.actions) {
            out.push(Number(action.amount));
        }
        return out;
    };

    const isSelectedTerminalMut8 = (inputs: SelectData) => {
        if (inputs.locked || inputs.selectedIdx === -1) {
            return false;
        }
        const selectedNode = inputs.nodes[inputs.selectedIdx];
        return selectedNode.type === "terminal";
    };

    const maxAmountMut8 = (inputs: SelectData) => {
        if (isSelectedTerminalMut8(inputs)) return 0;
        const maxTotalBetAmount = Math.max(...inputs.totalBet);
        return config.effectiveStack - (maxTotalBetAmount - inputs.prevBet);
    };

    const minAmountMut8 = (inputs: SelectData) => {
        const betMinus = config.effectiveStack - maxAmountMut8(inputs);
        const min = Math.min(...inputs.totalBet) - betMinus;
        const max = Math.max(...inputs.totalBet) - betMinus;
        return Math.min(Math.max(2 * max - min, 1), maxAmountMut8(inputs));
    };

    const encodeLine = (nodeIdx: number, inputs: SelectData) => {
        const out = [];
        for (let i = 1; i < nodeIdx; ++i) {
            const node = inputs.nodes[i];
            if (node.type === "player") {
                const action = node.actions[node.selectedIdx];
                if (action.name === "Fold") {
                    out.push("F");
                } else if (action.name === "Call") {
                    out.push("C");
                } else if (action.name === "Check") {
                    out.push("X");
                } else if (action.name === "Bet") {
                    out.push("B" + action.amount);
                } else if (action.name === "Raise") {
                    out.push("R" + action.amount);
                } else if (action.name === "Allin") {
                    out.push("A" + action.amount);
                }
            }
        }
        return out;
    };

    const pushResultsPlayer = async (inputs: SelectData) => {
        
        const prevNode = inputs.nodes[inputs.selectedIdx - 1];
        const player = prevNode.player === "oop" ? "ip" : "oop";
        const actions = await rust.getActions();
        inputs.nodes.push(
            {
                type: "player",
                idx: inputs.selectedIdx,
                player: player,
                selectedIdx: -1,
                actions: actions.map((action, i) => {
                    const [name, amount] = action.split(":");
                    return {
                        idx: i,
                        name,
                        amount,
                        selected: false,
                        colour: "#000"
                    }
                }),
            }
        );
    };

    const pushResultsTerminal = (inputs: SelectData) => {
        const prevNode = inputs.nodes[inputs.selectedIdx - 1] as PlayerNode;
        const prevAction = prevNode.actions[prevNode.selectedIdx];

        let equityOOP = -1;
        if (prevAction.name === "Fold") equityOOP = prevNode.player === "oop" ? 0 : 1;

        inputs.nodes.push(
            {
                type: "terminal",
                idx: inputs.selectedIdx,
                player: "end",
                selectedIdx: -1,
                prevPlayer: prevNode.player,
                equityOOP,
                pot: config.startingPot + inputs.totalBet[0] + inputs.totalBet[1],
            },
        );
    };

    const pushResultsChance = async (inputs: SelectData) => {
        
        type TurnNode = RootNode | ChanceNode;
        const prevNode = inputs.nodes[inputs.selectedIdx - 1] as PlayerNode;
        const turnNode = inputs.nodes.find(node => node.player === "turn") as TurnNode | undefined;
        const nxtActions = await rust.getActions();

        inputs.nodes.push(
            {
                type: "chance",
                idx: inputs.selectedIdx,
                player: turnNode ? "river" : "turn",
                selectedIdx: -1,
                prevPlayer: prevNode.player,
                cards: Array.from({ length: 52}, (_, i) => ({
                    card: i,
                    selected: false,
                    dead: true,
                })),
                pot: config.startingPot + 2 * inputs.totalBet[0],
                stack: config.effectiveStack - inputs.totalBet[0],
            },
        );
        inputs.nodes.push(
            {
                type: "player",
                idx: inputs.selectedIdx + 1,
                player: "oop",
                selectedIdx: -1,
                actions: nxtActions.map((action, i) => {
                    const [name, amount] = action.split(":");
                    return {
                        idx: i,
                        name,
                        amount,
                        rate: -1,
                        selected: false,
                        colour: "#000"
                    };
                }),
            },
        );
    };

    const play = async (nodeIdx: number, actionIdx: number) => {
        const nodesCopy = [...nodes];
        const node = nodesCopy[nodeIdx] as PlayerNode;

        if (node.selectedIdx !== -1) {
            node.actions[node.selectedIdx].selected = false;
        }
        node.actions[actionIdx].selected = true;
        node.selectedIdx = actionIdx;

        const inputs: SelectData = {
            nodes: nodesCopy,
            selectedIdx: selectedNodeIdx,
            bet: betAmount,
            totalBet: totalBetAmount,
            prevBet: prevBetAmount,
            locked: locked,
        };
        selectNodeInternal(nodeIdx + 1, true, false, true, inputs).then(() => {
            setNodes(inputs.nodes);
            setSelectedNodeIdx(inputs.selectedIdx);
            setBetAmount(inputs.bet);
            setTotalBetAmount(inputs.totalBet);
            setPrevBetAmount(inputs.prevBet);
            setLocked(inputs.locked);
        });
    };

    
    
    const selectNode = async (
        nodeIdx: number,
        splice: boolean,
        rebuild: boolean,
        updateAmount: boolean
        ) => {

        const inputs: SelectData = {
            nodes: nodes,
            selectedIdx: selectedNodeIdx,
            bet: betAmount,
            totalBet: totalBetAmount,
            prevBet: prevBetAmount,
            locked: locked,
        };
        await selectNodeInternal(nodeIdx, splice, rebuild, updateAmount, inputs);
        
        setNodes(inputs.nodes);
        setSelectedNodeIdx(inputs.selectedIdx);
        setBetAmount(inputs.bet);
        setTotalBetAmount(inputs.totalBet);
        setPrevBetAmount(inputs.prevBet);
        setLocked(inputs.locked);
    };

    return (
        <>
        <div className="flex flex-row overflow-x-auto h-40 gap-2 p-1 whitespace-nowrap">
            {
                nodes.map(node => {
                    return (
                        <div 
                            key={node.idx} 
                            className="flex flex-col h-full p-1 justify-start border-2 rounded-md border-neutral"
                            onClick = {() => selectNode(node.idx, false, false, true)}    
                        >
                            {
                                node.type === "player" ? 
                                    <>
                                        <div className="p-1 font-semibold group-hover:opacity-100">
                                            { node.player.toUpperCase() }
                                        </div>
                                        <div className="flex-grow overfloy-y-auto">
                                            {
                                                node.actions.map((action, i) => {
                                                return (
                                                    <button
                                                            key={action.idx}
                                                            className="flex w-full px-1.5 rounded-md transition-colors hover:base-200"
                                                            onClick={() => play(node.idx, i) }
                                                        >
                                                            { action.name }
                                                            { action.amount }
                                                        </button>
                                                    )
                                                })
                                            }
                                        </div>
                                    </>
                                : node.type === "terminal" ?
                                    <>
                                        <div className="p-1 font-semibold group-hover:opacity-100">
                                            { node.player.toUpperCase() }
                                        </div>
                                        <div className="flex flex-col flex-grow items-center justify-evenly font-semibold">
                                            {
                                                (node.equityOOP === 0 || node.equityOOP === 1) ? (
                                                    <p> { ["IP", "OOP"][node.equityOOP] } Wins </p>
                                                ) : null
                                            }
                                            <div className="px-3">
                                                { node.pot }
                                            </div>
                                        </div>
                                    </>
                                : // CHANCE NODE
                                    <>
                                        <div className="p-1 font-semibold group-hover:opacity-100 opacity-70">
                                            { node.player.toUpperCase() }
                                        </div>
                                        <div className="flex flex-col flex-grow px-3 items-center justify-evenly font-semibold">
                                            <div className="group-hover:opacity-100 opacity-70">
                                                { node.pot }
                                                { node.stack }
                                            </div>
                                        </div>
                                    </>
                            }
                        </div>
                    )
                })
            }
        </div>
        
        <div className="flex flex-col gap-1">
            <p>Nodes length: {nodes.length}</p>
            <p>Selected Node Index: {selectedNodeIdx}</p>
            <p>BetAmount: {betAmount}</p>
            <p>TotalBetAmount: {totalBetAmount}</p>
            <p>PrevBetAmount: {prevBetAmount}</p>
            <p>Locked: {locked ? "true" : "false"}</p>
            <div className="divider"></div>
            {
                nodes.map(node => {
                    return (
                        <>
                        <p>Type: {node.type}</p>
                        <p>Index: {node.idx}</p>
                        <p>Player: {node.player}</p>
                        <p>Selected Index: {node.selectedIdx}</p>
                        <div className="divider"></div>
                        </>
                    )
                })
            }
        </div>
        </>
    )
}
