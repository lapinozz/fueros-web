import "../style/index.scss";

import { menu } from "./menu";
import Board from "./board"

async function main() {
    window.Module = await import("../pkg/index.js").catch(console.error);
    menu();

    const board = new Board(10, 10);
    document.body.appendChild(board.app.view);
}

main();

