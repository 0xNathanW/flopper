import "./Navbar.css";
import { useAppDispatch } from "./store/store";
import { setPanel } from "./store/features/stateSlice";

export default function NavBar() {
    const dispatch = useAppDispatch();

    return (
        <div id="navbar">
            <button className="nav-button" onClick={() => dispatch(setPanel("build"))}>Config</button>
            <button className="nav-button" onClick={() => dispatch(setPanel("preview"))}>Tree</button>
            <button className="nav-button" onClick={() => dispatch(setPanel("solver"))}>Solver</button>
        </div>
    )
}