import "../style/index.scss";

import "./wasm-loader";

import { menu } from "./menu";
import Board from "./board";

async function main() {

    menu();

    let cbs = new Callbacks();

    let edgeUpdate;
    let es;

    cbs.set_edges((edges) => {
        const memory = new DataView(shared_memory().buffer);

        const edgesPtr = memory.getUint32(edges.ptr + 4 + 0, true);
        const edgesLen = memory.getUint32(edges.ptr + 4 + 4, true);
        edgeUpdate = new EdgeUpdate(edgesPtr); 

        es = new EdgeUpdateArray(edges.ptr); 
    });

    let t1 = performance.now();
    run_game(cbs);
    let t2 = performance.now();
    console.log(`${t2 - t1} ms`);
    console.log(edgeUpdate)
    console.log(es)

    const board = new Board(10, 10);
    document.body.appendChild(board.app.view);
}

main();
