import { createSlice, PayloadAction } from "@reduxjs/toolkit";

export interface ConfigTree {

    startingPot:    number,
    effectiveStack: number,
    rake:           number,
    rakeCap:        number,
    addAllIn:       number,
    forceAllIn:     number,

    betSizes:       [
    // IP
        [
            [string, string], // flop [bet, raise]
            [string, string], // turn
            [string, string], // river
        ],
    // OOP
        [
            [string, string],
            [string, string],
            [string, string],
        ],
    ]

    board:          number[],
    // 169 long array of weights to be coverted in rust backend.
    rangeIP:        Array<number>, 
    rangeOOP:       Array<number>,

    // TODO
    addedLines:     string,
    removedLines:   string,
}

const initialState: ConfigTree = {
    startingPot:    40,
    effectiveStack: 100,
    rake:           0,
    rakeCap:        3,
    addAllIn:       0,
    forceAllIn:     0,

    betSizes:       [
        // OOP
        [["", ""], ["", ""], ["", ""]],
        // IP
        [["", ""], ["", ""], ["", ""]],
    ],

    board:          [],
    rangeIP:        Array(169).fill(0),
    rangeOOP:       Array(169).fill(0),
    
    addedLines:     "",
    removedLines:   "",
}

export const configSlice = createSlice({
    name: "config",
    initialState,
    reducers: {
        
        setStartingPot: (state, action: PayloadAction<number>) => {
            state.startingPot = action.payload;
        },
        setEffectiveStack: (state, action: PayloadAction<number>) => {
            state.effectiveStack = action.payload;
        },
        setRake: (state, action: PayloadAction<number>) => {
            state.rake = action.payload;
        },
        setRakeCap: (state, action: PayloadAction<number>) => {
            state.rakeCap = action.payload;
        },
        setAddAllIn: (state, action: PayloadAction<number>) => {
            state.addAllIn = action.payload;
        },
        setForceAllIn: (state, action: PayloadAction<number>) => {
            state.forceAllIn = action.payload;
        },

        // params: oop: bool, street: number, raise: boolean, betSize: string
        setBetSize(state, action: PayloadAction<[boolean, number, boolean, string]>) {
            const [oop, street, raise, betSize] = action.payload;
            state.betSizes[+oop][street][+raise] = betSize;
        },

        copyOOP(state) {
            state.betSizes[0] = state.betSizes[1];
        },

        // params: [idx, weight]
        setWeightOOP: (state, action: PayloadAction<[number, number]>) => {
            state.rangeOOP = state.rangeOOP.map((weight, idx) => idx === action.payload[0] ? action.payload[1] : weight)
        },
        setWeightIP: (state, action: PayloadAction<[number, number]>) => {
            state.rangeIP = state.rangeIP.map((weight, idx) => idx === action.payload[0] ? action.payload[1] : weight)
        },
        setRangeOOP: (state, action: PayloadAction<Array<number>>) => {
            state.rangeOOP = action.payload;
        },
        setRangeIP: (state, action: PayloadAction<Array<number>>) => {
            state.rangeIP = action.payload;
        },
        clearRangeIP: (state) => {
            state.rangeIP = Array(169).fill(0);
        },
        clearRangeOOP: (state) => {
            state.rangeOOP = Array(169).fill(0);
        },

        randomiseBoard(state, action: PayloadAction<number>) {
            let newBoard: number[] = [];
            for (let i = 0; i < action.payload; i++) {
                let card = Math.floor(Math.random() * 52);
                while (newBoard.includes(card)) {
                    card = Math.floor(Math.random() * 52);
                }
                newBoard.push(card);
            }
            state.board = newBoard;
        },
        addToBoard: (state, action: PayloadAction<number>) => {
            state.board.push(action.payload);
        },
        removeFromBoard: (state, action: PayloadAction<number>) => {
            state.board = state.board.filter((card) => card !== action.payload);
        },
        clearBoard: (state) => {
            state.board = [];
        },

        setAddedLines: (state, action: PayloadAction<string>) => {
            state.addedLines = action.payload;
        },
        setRemovedLines: (state, action: PayloadAction<string>) => {
            state.removedLines = action.payload;
        },
    }
})

export const { 
    setStartingPot, 
    setEffectiveStack, 
    setRake, 
    setRakeCap, 
    setAddAllIn, 
    setForceAllIn,
    setWeightOOP,
    setWeightIP,
    setBetSize,
    copyOOP,
    setRangeOOP,
    setRangeIP,
    clearRangeIP,
    clearRangeOOP,
    randomiseBoard,
    addToBoard,
    removeFromBoard,
    clearBoard,
    setAddedLines,
    setRemovedLines,
} = configSlice.actions;

export default configSlice.reducer;