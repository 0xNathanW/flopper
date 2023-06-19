import { useEffect, useState } from "react";
import { useAppDispatch, useAppSelector } from "../store/store";
import { clearRangeIP, clearRangeOOP, setRangeIP, setRangeOOP, setWeightIP, setWeightOOP } from "../store/features/configSlice";

const RANKS = ["A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2"];

const folder = <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-4 h-4">
<path strokeLinecap="round" strokeLinejoin="round" d="M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z" />
</svg>

const ricon = <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-4 h-4">
<path strokeLinecap="round" strokeLinejoin="round" d="M12 9v6m3-3H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
</svg>

export default function Range({ oop }: { oop: boolean }) {
    // Weight to be applied to selected cells.
    const [weight, setWeight] = useState(100);
    // Is mouse down within grid.
    const [mouseDown, setMouseDown] = useState(false);

    const config = useAppSelector(state => state.config);
    const dispatch = useAppDispatch();

    useEffect(() => {
        window.addEventListener("mouseup", handleMouseUp);
        return () => {
            window.removeEventListener("mouseup", handleMouseUp);
        }
    }, []);

    // If mouse is down and the mouse is over a cell, update cell weight.
    const handleMouseDown = (i: number, j: number) => {
        const idx = i * 13 + j;
        if (oop) {
            dispatch(setWeightOOP([idx, weight]));
        } else {
            dispatch(setWeightIP([idx, weight]));
        }
        setMouseDown(true);
    }
    
    // Same as above but enables dragging.
    const handleMouseOver = (i: number, j: number) => {
        if (mouseDown) {
            const idx = i * 13 + j;
            if (oop) {
                dispatch(setWeightOOP([idx, weight]));
            } else {
                dispatch(setWeightIP([idx, weight]));
            }
        }
    }

    // If mouse is up, set mouseDown to false.
    const handleMouseUp = () => {
        setMouseDown(false);
    }

    // Text relating to the hand.
    const cellText = (i: number, j: number) => {
        const rank1 = RANKS[i];
        const rank2 = RANKS[j];
        if (i === j) {
            return rank1 + rank2
        } else if (i < j) {
            return rank1 + rank2 + "s"
        } else {
            return rank1 + rank2 + "o"
        }
    }

    const getWeight = (i: number, j: number) => {
        return oop ? config.rangeOOP[i * 13 + j] : config.rangeIP[i * 13 + j];
    }

    // Don't show weight if it's 0 or 100.
    const showWeight = (i: number, j: number) => {
        return !(getWeight(i, j) === 0 || getWeight(i, j) === 100);
    }

    // Create the cells for the range grid.
    let cells = RANKS.map((_, i) => {
        let row = RANKS.map((_, j) => {
            
            const txt = cellText(i, j);
            return (
                <td
                    key = {txt}
                    className="relative border-2 border-primary-content w-12"
                    onMouseDown={ () => handleMouseDown(i, j) }
                    onMouseOver={ () => handleMouseOver(i, j) }
                >
                    
                    <div className="absolute top-0 left-0 w-full h-full bg-primary">
                        <div 
                            className="absolute w-full h-full left-0 top-0 bg-bottom bg-no-repeat"
                            style={{
                                backgroundImage: 
                                    `linear-gradient(to top, #a4cbb4, 
                                    #a4cbb4 ${getWeight(i, j)}%, 
                                    transparent ${getWeight(i, j)}%)`,
                            }}
                        ></div>
                    </div>
                    
                    <div className="absolute top-0 left-0.5">{txt}</div>
                    <div className="absolute bottom-0 right-0.5">{showWeight(i, j) ? getWeight(i, j) + "%" : ""}</div>
                </td>
            )
        })
        return (
            <tr key={"row" + i} className="h-12">
                {row}
            </tr>
        )
    });

    const handleLoadRange = (s: string) => {
        if (oop) {
            dispatch(setRangeOOP(textToRange(s)));
        } else {
            dispatch(setRangeIP(textToRange(s)));
        }
    }

    return (
        <div className="flex flex-row items-start justify-center gap-5">   
            <div className="flex flex-col items-start w-[625px] min-w-[625px]">
                <div className="flex flex-row mb-3 w-full justify-between items-center">
                    <h2 className="font-bold text-3xl">{oop ? "OOP" : "IP"} Range</h2>
                    <button 
                        className="btn btn-primary" 
                        onClick={() => {
                            if (oop) {
                                dispatch(clearRangeOOP());
                            } else {
                                dispatch(clearRangeIP());
                            }
                        }}
                    >Clear</button>
                </div>
                <table className="table-fixed select-none">
                    <tbody>
                        {cells}
                    </tbody>
                </table>
                <div className="flex flex-row mt-4 w-full items-center gap-5">
                    <p>Weight: </p>
                    <input 
                        className="range range-primary"
                        type="range" 
                        min="0" 
                        max="100" 
                        value={weight} 
                        onChange={(e) => setWeight(parseInt(e.target.value))} 
                    />
                    <input
                        className="input input-bordered input-primary input-sm"
                        type="number"
                        min="0"
                        max="100"
                        step="5"
                        value={weight}
                        onChange={(e) => setWeight(parseInt(e.target.value))}
                    />
                    <p>%</p>
                </div>
            </div>
            <div className="w-fit">
                
                <ul className="menu menu-sm bg-base-200 rounded-lg w-[300px] h-[625px] overflow-y-auto mt-16">
                    <li>
                        <details>
                            <summary>{folder}6-Max PokerStrategy.com</summary>
                            <ul>

                                <li>
                                    <details>
                                        <summary>{folder}Open Raise</summary>
                                        <ul>
                                            <li><a onClick={() => handleLoadRange("66+,AJs+,AQo+")}>{ricon}MP2</a></li>
                                            <li><a onClick={() => handleLoadRange("66+,AJs+,KQs,AJo+,KQo")}>{ricon}MP3</a></li>
                                            <li><a onClick={() => handleLoadRange("22+,A7s+,K9s+,Q9s+,J9s+,T8s+,A9o+,K9o+,QTo+,J9o+")}>{ricon}CO</a></li>
                                            <li><a onClick={() => handleLoadRange("22+,A2s+,K8s+,Q8s+,J7s+,T9s,98s,87s,76s,65s,54s,A9o+,K9o+,Q9o+,J8o+,T8o+,97o+,87o,76o,65o,54o")}>{ricon}BTN</a></li>
                                            <li><a onClick={() => handleLoadRange("22+,ATs+,KTs+,QTs+,JTs,ATo+,KTo+,QTo+,JTo")}>{ricon}SB</a></li>
                                        </ul>
                                    </details>
                                </li>

                                <li>
                                    <details>
                                        <summary>{folder}Open Raise - Tight</summary>
                                        <ul>
                                            <li><a onClick={() => handleLoadRange("66+,AJs+,AQo+")}>{ricon}MP2</a></li>
                                            <li><a onClick={() => handleLoadRange("66+,AJs+,KQs,AJo+,KQo")}>{ricon}MP3</a></li>
                                            <li><a onClick={() => handleLoadRange("22+,A7s+,K9s+,Q9s+,J9s+,T8s+,A9o+,K9o+,QTo+,J9o+")}>{ricon}CO</a></li>
                                            <li><a onClick={() => handleLoadRange("22+,A2s+,K8s+,Q8s+,J7s+,T9s,98s,87s,76s,65s,54s,A9o+,K9o+,Q9o+,J8o+,T8o+,97o+,87o,76o,65o,54o")}>{ricon}BTN</a></li>
                                            <li><a onClick={() => handleLoadRange("22+,ATs+,KTs+,QTs+,JTs,ATo+,KTo+,QTo+,JTo")}>{ricon}SB</a></li>
                                        </ul>
                                    </details>
                                </li>

                                <li>
                                    <details>
                                        <summary>{folder}Open Raise - Loose</summary>
                                        <ul>
                                            <li><a onClick={() => handleLoadRange("22+,ATs+,KJs+,QJs,JTs,T9s,98s,87s,76s,65s,ATo+,KJo+")}>{ricon}MP2</a></li>
                                            <li><a onClick={() => handleLoadRange("22+,A9s+,KJs+,QTs+,JTs,T9s,98s,87s,76s,65s,A9o+,KJo+,QJo")}>{ricon}MP3</a></li>
                                            <li><a onClick={() => handleLoadRange("22+,A2s+,K7s+,Q7s+,J7s+,T8s+,97s+,87s,76s,65s,54s,A2o+,K8o+,Q8o+,J8o+,T8o+,97o+,87o,76o,65o,54o")}>{ricon}CO</a></li>
                                            <li><a onClick={() => handleLoadRange("22+,A2s+,K2s+,Q2s+,J5s+,T8s+,97s+,87s,76s,65s,54s,A2o+,K2o+,Q8o+,J8o+,T8o+,97o+,87o,76o,65o,54o")}>{ricon}BTN</a></li>
                                            <li><a onClick={() => handleLoadRange("22+,A2s+,K7s+,Q7s+,J7s+,T8s+,97s+,87s,76s,65s,54s,A2o+,K8o+,Q8o+,J8o+,T8o+,97o+,87o,76o,65o,54o")}>{ricon}SB</a></li>
                                        </ul>
                                    </details>
                                </li>

                            </ul>
                        </details>
                    </li>
                </ul>
            </div>      
            
        </div>
    )
}

function textToRange(text: string): number[] {

    const weights = Array(169).fill(0);
    const elems = text
        .replace(/\s*([-:,])\s*/g, '$1')
        .split(',')
        .map(elem => elem.trim());
    
    for (let elem of elems) {
  
        if (elem.includes('+')) {
            const idx1 = RANKS.indexOf(elem[0]);
            const idx2 = RANKS.indexOf(elem[1]);
            
            // Pair+
            if (elem.length === 3) {
                for (let i = 0; i <= idx1; i++) {
                    weights[i * 13 + i] = 100;
                }
        
            // Suited+
            } else if (elem[2] === 's') {
                for (let i = idx1; i <= idx2; i++) {
                    weights[idx1 * 13 + i] = 100;
                }
            
              // Offsuit+
            } else if (elem[2] === 'o') {
                for (let i = idx1 + 1; i <= idx2; i++) {
                    weights[i * 13 + idx1] = 100;
                }
            }

        } else if (elem.includes('-')) {
            
            const split = elem.split('-');

            const idx11 = RANKS.indexOf(split[0][0]);
            const idx12 = RANKS.indexOf(split[0][1]);

            const idx21 = RANKS.indexOf(split[1][0]);
            const idx22 = RANKS.indexOf(split[1][1]);

            // Pair-Pair
            if (split[0].length === 2) {
                for (let i = idx11; i <= idx21; i++) {
                    weights[i * 13 + i] = 100;
                }
            }

            // Suited-Suited
            else if (split[0][2] === 's') {
                for (let i = idx11; i <= idx21; i++) {
                    for (let j = idx22; j <= idx12; j++) {
                        weights[i * 13 + j] = 100;
                    }
                }
            }

            // Offsuit-Offsuit
            else if (split[0][2] === 'o') {
                for (let i = idx11; i <= idx21; i++) {
                    for (let j = idx12; j <= idx22; j++) {
                        weights[i * 13 + j] = 100;
                    }
                }
            }
        } else {

            const idx1 = RANKS.indexOf(elem[0]);
            const idx2 = RANKS.indexOf(elem[1]);

            // Pair
            if (elem.length === 2) {
                weights[idx1 * 13 + idx1] = 100;
            }

            // Suited
            else if (elem[2] === 's') {
                weights[idx1 * 13 + idx2] = 100;
            }

            // Offsuit
            else if (elem[2] === 'o') {
                weights[idx2 * 13 + idx1] = 100;
            }
        }
    }
    return weights;
}