import * as PIXI from "pixi.js";
import Array2D from "./Array2D";

class Board extends Array2D {
    constructor() {
        super(...arguments);

        this.app = new PIXI.Application({
            backgroundColor: 0x1099bb,
        });

        this.forEach((v, x, y) => {
            this[x][y] = Math.floor(Math.random() * 3);
        });

        const caseGraphics = new PIXI.Graphics();
        const backEdgeGraphics = new PIXI.Graphics();
        const frontEdgeGraphics = new PIXI.Graphics();

        this.app.stage.addChild(caseGraphics);
        this.app.stage.addChild(backEdgeGraphics);
        this.app.stage.addChild(frontEdgeGraphics);

        const caseColors = [0xff0000, 0x0000ff, 0x00ff00];
        const caseSize = 50;
        this.forEach((v, x, y) => {
            caseGraphics.beginFill(caseColors[this[x][y]]);
            caseGraphics.drawRect(
                caseSize * x,
                caseSize * y,
                caseSize,
                caseSize
            );
            caseGraphics.endFill();
        });

        console.log(this);

        const edgeSize = 4;
        const halfEdge = edgeSize / 2;
        const round = halfEdge;

        const backEdgeColor = 0x404040;
        const frontEdgeColor = 0;

        const drawEdge = (x, y, horizontal = true) => {
            if (horizontal ? y >= this.y : x >= this.x) {
                return;
            }

            const dh = horizontal ? 1 : 0;
            const dv = horizontal ? 0 : 1;

            const inBound = this.inBounds(x - dh, y - dv, 1 + dh, 1 + dv);
            const isSame = inBound && this[x - dh][y - dv] == this[x][y];
            const isDefault = isSame && this[x][y] == 0;

            if (!isSame || isDefault) {
                const graphics = isDefault
                    ? backEdgeGraphics
                    : frontEdgeGraphics;
                graphics.beginFill(isDefault ? backEdgeColor : frontEdgeColor);
                graphics.drawRoundedRect(
                    caseSize * x - halfEdge,
                    caseSize * y - halfEdge,
                    edgeSize + caseSize * dv,
                    edgeSize + caseSize * dh,
                    round
                );
                graphics.endFill();
            }
        };

        for (let x = 0; x <= this.x; x++) {
            for (let y = 0; y <= this.y; y++) {
                drawEdge(x, y, false);
                drawEdge(x, y, true);
            }
        }
    }
}

export default Board;
