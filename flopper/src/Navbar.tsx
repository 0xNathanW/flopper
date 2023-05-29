import "./Navbar.css";
import { useAppDispatch, useAppSelector } from "./store/store";
import { setPanel } from "./store/features/stateSlice";

export default function NavBar() {

    const dispatch = useAppDispatch();
    const panel = useAppSelector(state => state.appState.panel);

    return (
        <div id="navbar">
            <button 
                className="nav-button"
                onClick={() => dispatch(setPanel("build"))}
                style={{borderBottom: panel === "build" ? "5px solid #FFD166" : ""}}
            >Config</button>
            
            <button 
                className="nav-button" 
                onClick={() => dispatch(setPanel("preview"))}
                style={{borderBottom: panel === "preview" ? "5px solid #FFD166" : ""}}
            >Tree</button>
            
            <button 
                className="nav-button" 
                onClick={() => dispatch(setPanel("solver"))}
                style={{borderBottom: panel === "solver" ? "5px solid #FFD166" : ""}}
            >Solver</button>
        </div>
    )
}