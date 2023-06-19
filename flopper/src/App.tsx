import "./index.css";
import { useState } from "react";
import Navbar from "./components/Navbar";
import Config from "./components/Config";

function App() {

    const [panel, setPanel] = useState("config");

    return (
        <>
            <Navbar setPanel={setPanel} />
            <div>
                { panel === "config" ? <Config /> : <Config /> }
            </div>
        </>
    )
}

export default App;

