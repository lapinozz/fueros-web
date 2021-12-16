import "../style/index.scss";

import { menu } from "./menu";
import { test } from "./test";
import Board from "./board";

async function main() {
    window.Module = await import("../pkg/index.js").catch(console.error);
    for (const prop of Object.getOwnPropertyNames(Module)) {
        window[prop] = Module[prop];
    }

    //test();
    menu();

    const board = new Board(10, 10);
    document.body.appendChild(board.app.view);
}

main();
