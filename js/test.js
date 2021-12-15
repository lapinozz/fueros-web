import { Vector2i } from "../pkg";
import { Board } from "../pkg";

export function test() {
    let board = Board.with_size(7, 7);
    console.log("board: ", board);
    board.set_cell(new Vector2i(0, 0), { Claimed: "System" });
    let cell = board.get_cell(new Vector2i(0, 0));
    console.log("cell: ", cell);
}