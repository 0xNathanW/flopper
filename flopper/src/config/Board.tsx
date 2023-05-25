import { RANKS  } from "../common";
import "./Board.css";

export default function Board({board, setBoard}: {board: number[], setBoard: (board: number[]) => void}) {

    const onClick = (e: React.MouseEvent<HTMLButtonElement>, rank: string, suit: string) => {
        let button = e.target as HTMLButtonElement;
        const idx = cardToIdx(rank, suit);
        if (board.includes(idx)) {
            button.style.backgroundColor = suitColour(suit);
            setBoard(board.filter(i => i !== idx));
        }
        else if (board.length < 5) {
            button.style.backgroundColor = "yellow";
            setBoard([...board, idx]);
        }
    }

    const boardText = board.map(idx => idxToCard(idx).join("")).join(" ");

    return (
        <div id="board">
            <h1 id="board-h1">Board: {boardText} </h1>
            <SuitCards suit={"♦"} onClick={onClick} />
            <SuitCards suit={"♥"} onClick={onClick} />
            <SuitCards suit={"♣"} onClick={onClick} />
            <SuitCards suit={"♠"} onClick={onClick} />
        </div>
    )
}

function SuitCards({suit, onClick}: {suit: string, onClick: (e: React.MouseEvent<HTMLButtonElement>, rank: string, suit: string) => void}) {
    return (
        <div className="suit-row">
            {
                RANKS.map((rank, i) => {
                    return BoardCard(rank, suit, onClick);
                })
            }
        </div>
    )
}

function BoardCard(rank: string, suit: string, onClick: (e: React.MouseEvent<HTMLButtonElement>, rank: string, suit: string) => void) {
    return (
        <button 
            className={`card ${suit}`}
            key={rank + suit}
            onClick={(e) => onClick(e,rank, suit)}
        >{rank}{suit[0]}</button>
    )
}

function cardToIdx(rank: string, suit: string) {
    return (12 - RANKS.indexOf(rank))* 4 + "♣♦♥♠".indexOf(suit);
}

function idxToCard(idx: number) {
    const rank = RANKS[12 - Math.floor(idx / 4)];
    const suit = "♣♦♥♠"[idx % 4];
    return [rank, suit];
}

function suitColour(suit: string): string {
    if (suit === "♦") {
        return "lightskyblue";
    } else if (suit === "♣") {
        return "lightgreen";
    } else if (suit === "♥") {
        return "lightcoral";
    } else {
        return "lightgrey";
    }
}