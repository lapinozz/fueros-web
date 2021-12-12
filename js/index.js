import * as PIXI from "pixi.js";
import Array2D from "./Array2D.js";

import "../style/index.scss";
import bunnyPng from "../bunny.png";

async function main() {
    window.Module = await import("../pkg/index.js").catch(console.error);
    menu();
}

main();

const app = new PIXI.Application({
    backgroundColor: 0x1099bb,
});

// The application will create a canvas element for you that you
// can then insert into the DOM.
document.body.appendChild(app.view);

const board = new Array2D(10, 10);
board.forEach((v, x, y) => {
    board[x][y] = Math.floor(Math.random() * 3);
});

const graphics = new PIXI.Graphics();

const caseColors = [0xff0000, 0x0000ff, 0x00ff00];
const caseSize = 40;
board.forEach((v, x, y) => {
    graphics.beginFill(caseColors[board[x][y]]);
    graphics.drawRect(caseSize * x, caseSize * y, caseSize, caseSize);
    graphics.endFill();
});

console.log(board);

const edgeSize = 5;
for (let x = 0; x <= board.x; x++) {
    for (let y = 0; y <= board.y; y++) {
        if (y < board.y) {
            const inBound = board.inBounds(x - 1, y, 2, 1);
            const isSame = inBound && board[x - 1][y] == board[x][y];
            const isDefault = isSame && board[x][y] == 0;

            if (!isSame || isDefault) {
                graphics.beginFill(isDefault ? 0x404040 : 0);
                graphics.drawRect(
                    caseSize * x - edgeSize / 2,
                    caseSize * y,
                    edgeSize,
                    caseSize
                );
                graphics.endFill();
            }
        }
        if (x < board.x) {
            const inBound = board.inBounds(x, y - 1, 1, 1);
            const isSame = inBound && board[x][y - 1] == board[x][y];
            const isDefault = isSame && board[x][y] == 0;

            if (!isSame || isDefault) {
                graphics.beginFill(isDefault ? 0x404040 : 0);
                graphics.drawRect(
                    caseSize * x,
                    caseSize * y - edgeSize / 2,
                    caseSize,
                    edgeSize
                );
                graphics.endFill();
            }
        }
    }
}

app.stage.addChild(graphics);
