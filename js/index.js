import "../style/index.scss";

import { menu } from "./menu";
import Board from "./board";

async function main() {
    window.Module = await import("../pkg/index.js").catch(console.error);
    for(const prop of Object.getOwnPropertyNames(Module))
    {
    	window[prop] = Module[prop];
    }

    let e = JsEdge.Set(10);

    // this is a method on the rust enum
    e.change_player(5);

    // prints 5
    console.log(e.Set_player_idx);

    menu();

    const board = new Board(10, 10);
    document.body.appendChild(board.app.view);
}

main();
