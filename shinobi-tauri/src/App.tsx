import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Board from "./components/Board";

function App() {
    return <div id="app"><Board /></div>
}

export default App;
