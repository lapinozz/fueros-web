import "../style/index.scss";

import { menu } from "./menu";
import { lobby } from "./lobby";
import Board from "./board";

async function main() {
    lobby();

    //const board = new Board(10, 10);
    //document.body.appendChild(board.app.view);
}

main();
