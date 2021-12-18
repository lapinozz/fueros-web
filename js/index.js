import "../style/index.scss";

import "./wasm-loader";

import { menu } from "./menu";
import Board from "./board";

async function main() {

    menu();

    let cbs = new window.Module.Callbacks();
    cbs.set_edges((edges) => {
        console.log(edges);
    });
    let t1 = performance.now();
    window.Module.run_game(cbs);
    let t2 = performance.now();
    console.log(`${t2 - t1} ms`);

    const board = new Board(10, 10);
    document.body.appendChild(board.app.view);
}

main();
