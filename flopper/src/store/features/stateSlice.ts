import { PayloadAction, createSlice } from "@reduxjs/toolkit";

export interface AppState {
    // Whether action tree has been built.
    treeBuilt: boolean,
    // Whether the solver has been run.
    solverRun: boolean,
}

const initialState: AppState = {
    treeBuilt: false,
    solverRun: false,
}

export const appSlice = createSlice({
    name: "app-state",
    initialState,
    reducers: {
        setTreeBuilt: (state, action: PayloadAction<boolean>) => {
            state.treeBuilt = action.payload;
        },
        setSolverRun: (state, action: PayloadAction<boolean>) => {
            state.solverRun = action.payload;
        },
    }
})

export const { setTreeBuilt, setSolverRun } = appSlice.actions;
export default appSlice.reducer;
