import { createSlice, PayloadAction } from "@reduxjs/toolkit";

export interface ActionTree {
}

const initialState: ActionTree = {

}

export const actionSlice = createSlice({
    name: "action-tree",
    initialState,
    reducers: {

    }
})

export const {  } = actionSlice.actions;

export default actionSlice.reducer;