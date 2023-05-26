import { createSlice, PayloadAction } from "@reduxjs/toolkit"

export interface Config {

    startingPot:    number,
    effectiveStack: number,
    rake:           number,
    rakeCap:        number,
    addAllIn:       number,
    forceAllIn:     number,

    board:          number[],
    rangeIP:        Array<number>,
    rangeOOP:       Array<number>,
}

const initialState: Config = {
    startingPot:    40,
    effectiveStack: 100,
    rake:           0,
    rakeCap:        3,
    addAllIn:       0,
    forceAllIn:     0,

    board:          [],
    rangeIP:        Array(169).fill(0),
    rangeOOP:       Array(169).fill(0),
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

        addToBoard: (state, action: PayloadAction<number>) => {
            state.board.push(action.payload);
        },
        removeFromBoard: (state, action: PayloadAction<number>) => {
            state.board = state.board.filter((card) => card !== action.payload);
        }
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
    setRangeOOP,
    setRangeIP,
    clearRangeIP,
    clearRangeOOP,
    addToBoard,
    removeFromBoard,
} = configSlice.actions;

export default configSlice.reducer;