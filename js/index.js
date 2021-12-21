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

function Test2(obj)
{
    let t = {}
    Object.assign(t, obj)
    return t;
}

async function main() {

    menu();

    const benchmarkCount = 100;

    let rustTime = 0;
    let edgeUpdate;
    let es;
    for(let i = 0; i < benchmarkCount; i++)
    {
        let cbs = new Callbacks();


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
        const time = t2 - t1;
        rustTime += time / benchmarkCount;
        //console.log(`${t2 - t1} ms`);
        //console.log(edgeUpdate)
        //console.log(es)

    }

    //const board = new Board(10, 10);
    //document.body.appendChild(board.app.view);


    let jsTime = 0;
    let a = [];
    for(let i = 0; i < benchmarkCount; i++)
    {
        let tl1 = performance.now();
        a = [];
        for(let i = 0; i < 100000; i++)
        {
            a.push(new Test({x: i, y: i % 10, set: !!(i % 2)}));
        }
        let tl2 = performance.now();
        const time = tl2 - tl1;
        jsTime += time / benchmarkCount;
        //console.log(`${tl2 - tl1} ms`);
        //console.log(a)
    }

    console.log({rustTime, jsTime});
    console.log(rustTime /  jsTime, rustTime - jsTime);

    console.log(es);

}

main();
