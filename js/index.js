import "../style/index.scss";

import "./wasm-loader";

import { menu } from "./menu";
import Board from "./board";
import { readStruct, readValue } from "./wasm-loader";

async function main() {
    menu();

    let boardData = new BoardData();
    let rawBoard = boardData.raw();
    const ptr = rawBoard.sptr;

    let t1 = performance.now();
    update_edges(boardData);
    let t2 = performance.now();
    console.log(`${t2 - t1} ms`);

    let memory = new DataView(shared_memory().buffer);
    let edge = readStruct("Edge", memory, ptr, 4);

    const board = new Board(10, 10);
    document.body.appendChild(board.app.view);
}

main();
