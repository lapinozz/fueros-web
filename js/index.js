import "../style/index.scss";

import "./wasm-loader";

import { menu } from "./menu";
import Board from "./board";

class Test
{
    constructor(obj)
    {
        Object.assign(this, obj)
    }
}

async function main() {

    menu();

    let cbs = new Callbacks();

    let edgeUpdate;
    let es;

    cbs.set_edges((edges) => {
        const memory = new DataView(shared_memory().buffer);

        const edgesPtr = memory.getUint32(edges.ptr + 4 + 0, true);
        const edgesLen = memory.getUint32(edges.ptr + 4 + 4, true);
        edgeUpdate = new EdgeUpdate(edgesPtr, memory); 

        es = new EdgeUpdateArray(edges.ptr, memory); 
    });

    let t1 = performance.now();
    run_game(cbs);
    let t2 = performance.now();
    console.log(`${t2 - t1} ms`);
    console.log(edgeUpdate)
    console.log(es)

    const board = new Board(10, 10);
    document.body.appendChild(board.app.view);


    let tl1 = performance.now();
    let a = [];
    for(let i = 0; i < 100000; i++)
    {
        a.push(new Test({x: i, y: i % 10, set: !!(i % 2)}));
    }
    let tl2 = performance.now();
    console.log(`${tl2 - tl1} ms`);
    console.log(a)

}

main();
