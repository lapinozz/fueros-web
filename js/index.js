import "../style/index.scss";

import { menu } from "./menu";
import Board from "./board";

async function main() {
    window.Module = await import("../pkg/index.js").catch(console.error);
    for (const prop of Object.getOwnPropertyNames(Module)) {
        window[prop] = Module[prop];
    }

    menu();

    let cbs = new window.Module.Callbacks();
    cbs.set_edges((edges) => {
        /* do something with edges */
    });
    let t1 = performance.now();
    window.Module.run_game(cbs);
    let t2 = performance.now();
    console.log(`${t2 - t1} ms`);

    const board = new Board(10, 10);
    document.body.appendChild(board.app.view);
}

main();
