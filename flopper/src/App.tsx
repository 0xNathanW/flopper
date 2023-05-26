import "./App.css";
import NavBar from "./Navbar";
import Config from "./config/Config";
import Preview from "./preview/Preview";
import { useAppSelector } from "./store/store";

function App() {

    const appState = useAppSelector(state => state.appState);

    const panel = () => {
        if (appState.panel === "build") {
            return <Config />;
        } else if (appState.panel === "preview") {
            return <Preview />;
        } else {
            return <Config />;
        }
    }

    return (
        <>
            <NavBar />
            {panel()}
        </>
  );
}

export default App;
