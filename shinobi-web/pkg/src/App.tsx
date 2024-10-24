import { createSignal } from "solid-js";
import "./App.css";
import Board from "./components/Board";
import init, { multiply, ClientEngine } from './../shinobi_web.js';
import { Client } from "@tauri-apps/api/http";

const wasm = await init(); // Initializes the WASM module
// You can now use functions from the WASM module
console.log("WASM INITIALZIED");
let engine = new ClientEngine();


function App() {
    return <div id="app"><Board engine={engine} /></div>
}

export default App;
