import { addToBoard, clearBoard, randomiseBoard, removeFromBoard } from "../store/features/configSlice";
import { useAppDispatch, useAppSelector } from "../store/store";

const RANKS = ["A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2"];

export default function Board() {
    
    const board = useAppSelector(state => state.config.board);
    const dispatch = useAppDispatch();

    const handleClick = (rank: string, suit: string) => {
        const idx = cardToIdx(rank, suit);
        if (board.includes(idx)) {
            dispatch(removeFromBoard(idx));
        }
        else if (board.length < 5) {
            dispatch(addToBoard(idx));
        }
    }

    return (
        <div className="flex flex-col items-center">
            <div className="flex flex-col items-center w-fit-content gap-4">
                <div className="flex flex-row items-center gap-4 self-start">
                    <h1 className="text-2xl font-bold">Board:</h1>
                    <BoardText board={board} />
                </div>
                <div className="grid grid-cols-[repeat(13,50px)] gap-2">
                    <SuitCards suit={"♦"} onClick={handleClick} />
                    <SuitCards suit={"♥"} onClick={handleClick} />
                    <SuitCards suit={"♣"} onClick={handleClick} />
                    <SuitCards suit={"♠"} onClick={handleClick} />
                </div>
                <div className="flex flex-row justify-between w-full">
                    <button className="btn btn-primary" onClick={() => dispatch(clearBoard())}>Clear</button>
                    <div className="flex flex-row items-center gap-3">
                        <p className="text-xl">Randomise:</p>
                        <div className="btn-group">
                            <button className="btn btn-primary" onClick={() => dispatch(randomiseBoard(3))}>Flop</button>
                            <button className="btn btn-primary" onClick={() => dispatch(randomiseBoard(4))}>Turn</button>
                            <button className="btn btn-primary" onClick={() => dispatch(randomiseBoard(5))}>River</button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    )
}

export function BoardText({board}: {board: number[]}) {
    
    const colour = (suit: string) => {
        if (suit === "♦") {
            return "blue";
        } else if (suit === "♣") {
            return "green";
        } else if (suit === "♥") {
            return "red";
        } else {
            return "black";
        }
    }

    return (
        <h1>
            {
                board.map(idx => {
                    const [rank, suit] = idxToCard(idx);
                    return <span key={idx} className={`text-${colour(suit)}-500 text-2xl font-bold`}>{rank}{suit[0]} </span>
                })
            }
        </h1>
    )

}

function SuitCards({suit, onClick}: {suit: string, onClick: (rank: string, suit: string) => void}) {
    return (
        <>
            {
                RANKS.map((rank, i) => {
                    return BoardCard(rank, suit, onClick);
                })
            }
        </>
    )
}

function BoardCard(rank: string, suit: string, onClick: (rank: string, suit: string) => void) {

    const board = useAppSelector(state => state.config.board);

    return (
        <button 
            className="h-12 w-12 outline-none border-2 border-neutral rounded-lg text-xl select-none"
            style={{backgroundColor: board.includes(cardToIdx(rank, suit)) ? "yellow" : suitColour(suit)}}
            key={rank + suit}
            onClick={(e) => onClick(rank, suit)}
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