import { useState } from "react";
import * as rust from "../rust_funcs";
import { useAppDispatch, useAppSelector } from "../store/store";
import MiniRange from "./MiniRange";
import Board from "./Board";
import { BoardText } from "./Board";
import RunSolver from "./RunSolver";
import TreeConfig from "./TreeConfig";
import Range from "./Range";
import { setTreeBuilt } from "../store/features/stateSlice";

export default function Config() {

    const [panel, setPanel] = useState("RangeOOP");
    const config = useAppSelector(state => state.config);
    const appState = useAppSelector(state => state.appState);
    const dispatch = useAppDispatch();

    const renderPanel = () => {
        switch (panel) {
            case "RangeOOP":
                return <Range oop={true} />
            case "RangeIP":
                return <Range oop={false} />
            case "Board":
                return <Board />
            case "tree-config":
                return <TreeConfig />
            case "run":
                return <RunSolver />
            default:
                // Isn't really necessary but typescript complains otherwise.
                return null;
        }
    }

    const isConfigValid = () => {
        // Check empty ranges
        if (config.rangeOOP.reduce((sum, p) => sum + p) === 0 || config.rangeIP.reduce((sum, p) => sum + p) === 0) { 
            return false;
        }
        if (config.board.length < 3) {
            return false;
        }
        // Check bet sizes
        const errSizes = ["", "error"]
        for (let i = 0; i < 2; i++) {
            for (let j = 0; j < 3; j++) {
                for (let k = 0; k < 2; k++) {
                    if (config.betSizes[i][j][k] === errSizes[k]) {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    return (
        <>
            {/* sidebar */}
            <div className="drawer absolute lg:drawer-open">
                <input id="my-drawer-2" type="checkbox" className="drawer-toggle" />
                <div className="drawer-content mt-7 px-5 justify-center">
                    {renderPanel()}
                    <label htmlFor="my-drawer-2" className="btn btn-base-200 drawer-button lg:hidden self-start ml-3 fixed top-16 left-2">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
                            <path strokeLinecap="round" strokeLinejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25H12" />
                        </svg>
                    </label>
                </div> 
                <div className="drawer-side top-auto">
                    <label htmlFor="my-drawer-2" className="drawer-overlay"></label>
                    <ul className="menu p-4 w-75 h-full bg-base-200 text-base-content text-xl">
                        {/* Sidebar content here */}
                        <li onClick={() => setPanel("RangeOOP")}><MiniRange oop={true} /></li>
                        <li onClick={() => setPanel("RangeIP")}><MiniRange oop={false} /></li>
                        <li onClick={() => setPanel("Board")}><a>
                            <div className="flex flex-col gap-1">
                                Board
                                {
                                    config.board.length >= 3 ?
                                        <BoardText board={config.board} />
                                        :
                                        <div className="badge badge-warning badge-md mt-1">3 cards minimum required!</div>
                                }
                            </div>
                        </a></li>
                        <li onClick={() => setPanel("tree-config")}><a>Tree Config</a></li>
                        <div className="divider"></div>
                        <button 
                            className="btn btn-outline text-xl font-normal capitalize w-full text-left"
                            disabled={!isConfigValid() }
                            onClick={async () => {
                                await rust.newActionTree(
                                    config.board,
                                    config.startingPot,
                                    config.effectiveStack,
                                    config.rake,
                                    config.rakeCap,
                                    config.addAllIn,
                                    config.forceAllIn,
                                    config.betSizes[0][0][0],
                                    config.betSizes[0][0][1],
                                    config.betSizes[0][1][0],
                                    config.betSizes[0][1][1],
                                    config.betSizes[0][2][0],
                                    config.betSizes[0][2][1],
                                    config.betSizes[1][0][0],
                                    config.betSizes[1][0][1],
                                    config.betSizes[1][1][0],
                                    config.betSizes[1][1][1],
                                    config.betSizes[1][2][0],
                                    config.betSizes[1][2][1],
                                ).then(() => {
                                    dispatch(setTreeBuilt(true));
                                    setPanel("run");
                                });
                            }}
                        >
                        Build & Run
                        </button>
                    </ul>

                </div>
            </div>
        </>
    )
}

