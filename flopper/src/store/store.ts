import { configureStore } from "@reduxjs/toolkit";
import { configSlice } from "./features/configSlice";
import { TypedUseSelectorHook, useDispatch, useSelector } from "react-redux";
import { appSlice } from "./features/stateSlice";

export const store = configureStore({
    reducer: {
        // Config for the solver.
        config:   configSlice.reducer,
        // Tracks the state of the app.
        appState: appSlice.reducer,
    },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export const useAppDispatch: () => typeof store.dispatch = useDispatch;
export const useAppSelector: TypedUseSelectorHook<RootState> = useSelector;
