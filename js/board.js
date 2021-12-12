import * as PIXI from "pixi.js";
import Array2D from "./Array2D";

class Board extends Array2D
{
    constructor()
    {
        super(...arguments);

        this.app = new PIXI.Application({
            backgroundColor: 0x1099bb,
        });

        this.forEach((v, x, y) => {
            this[x][y] = Math.floor(Math.random() * 3);
        });

        const graphics = new PIXI.Graphics();

        const caseColors = [0xff0000, 0x0000ff, 0x00ff00];
        const caseSize = 40;
        this.forEach((v, x, y) => {
            graphics.beginFill(caseColors[this[x][y]]);
            graphics.drawRect(caseSize * x, caseSize * y, caseSize, caseSize);
            graphics.endFill();
        });

        console.log(this);

        const edgeSize = 5;
        for (let x = 0; x <= this.x; x++) {
            for (let y = 0; y <= this.y; y++) {
                if (y < this.y) {
                    const inBound = this.inBounds(x - 1, y, 2, 1);
                    const isSame = inBound && this[x - 1][y] == this[x][y];
                    const isDefault = isSame && this[x][y] == 0;

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
                if (x < this.x) {
                    const inBound = this.inBounds(x, y - 1, 1, 1);
                    const isSame = inBound && this[x][y - 1] == this[x][y];
                    const isDefault = isSame && this[x][y] == 0;

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

        this.app.stage.addChild(graphics);
    }
};

export default Board;
