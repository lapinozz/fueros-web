import "../style/index.scss";

import "./wasm-loader";

import { menu } from "./menu";
import { test } from "./test";
import Board from "./board";

async function main() {

    //test();
    menu();

    const board = new Board(10, 10);
    document.body.appendChild(board.app.view);
}

main();
