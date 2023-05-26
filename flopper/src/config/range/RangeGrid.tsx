import { useEffect, useState } from "react";
import { RANKS } from "../../common";
import "./RangeGrid.css";
import { useAppDispatch, useAppSelector } from "../../store/store";
import { setWeightOOP, setWeightIP } from "../../store/features/configSlice";

const yellow500 = "#C14B1F";

export default function RangeGrid({oop, weight}: {oop: boolean, weight: number}) {

    const config = useAppSelector(state => state.config);
    const dispatch = useAppDispatch();

    // True if the mouse is down, false otherwise.
    const [mouseDown, setMouseDown] = useState<boolean>(false);

    const get_weight = (i: number, j: number) => {
        return oop ? config.rangeOOP[i * 13 + j] : config.rangeIP[i * 13 + j];
    }

    // Add event listeners for mouseup and mousedown.
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

    // Don't show weight if it's 0 or 100.
    const showWeight = (i: number, j: number) => {
        return !(get_weight(i, j) === 0 || get_weight(i, j) === 100);
    }

    // Create the cells for the range grid.
    let cells = RANKS.map((_, i) => {
        let row = RANKS.map((_, j) => {
            
            const txt = cellText(i, j);
            return (
                <td
                    key = {txt}
                    className="cell"
                    onMouseDown={ () => handleMouseDown(i, j) }
                    onMouseOver={ () => handleMouseOver(i, j) }
                >
                    
                    <div className="cell-background">
                        <div 
                            className="cell-weight"
                            style={{
                                backgroundImage: 
                                    `linear-gradient(to top, ${yellow500}, 
                                    ${yellow500} ${get_weight(i, j)}%, 
                                    transparent ${get_weight(i, j)}%)`,
                            }}
                        ></div>
                    </div>
                    
                    <div className="cell-hand-text">{txt}</div>
                    <div className="cell-weight-text">{showWeight(i, j) ? get_weight(i, j) + "%" : ""}</div>
                </td>
            )
        })
        return (
            <tr key={"row" + i}>
                {row}
            </tr>
        )
    })

    return (
        <table className="range-grid">
            <tbody>
                {cells}
            </tbody>
        </table>
    )
}