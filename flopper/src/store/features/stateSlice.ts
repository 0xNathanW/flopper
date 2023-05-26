import { PayloadAction, createSlice } from "@reduxjs/toolkit";

export interface AppState {
    panel: string,
    treeBuilt: boolean,
    solverRun: boolean,
}

const initialState: AppState = {
    panel: "build",
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
        setPanel: (state, action: PayloadAction<string>) => {
            state.panel = action.payload;
        }
    }
})

export const { setTreeBuilt, setSolverRun, setPanel } = appSlice.actions;

export default appSlice.reducer;
