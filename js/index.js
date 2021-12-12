import * as PIXI from 'pixi.js';

import '../style/index.scss';
import bunnyPng from '../bunny.png';

async function main()
{
    window.Module = await import("../pkg/index.js").catch(console.error);
}

main();


const app = new PIXI.Application();
 
// The application will create a canvas element for you that you
// can then insert into the DOM.
document.body.appendChild(app.view);
 
// load the texture we need
app.loader.add('bunny', bunnyPng).load((loader, resources) => {
 
    // This creates a texture from a 'bunny.png' image.
    const bunny = new PIXI.Sprite(resources.bunny.texture);
 
    // Setup the position of the bunny
    bunny.x = app.renderer.width / 2;
    bunny.y = app.renderer.height / 2;
 
    // Rotate around the center
    bunny.anchor.x = 0.5;
    bunny.anchor.y = 0.5;
 
    // Add the bunny to the scene we are building.
    app.stage.addChild(bunny);
 
    // Listen for frame updates
    app.ticker.add(() => {
         // each frame we spin the bunny around a bit
        bunny.rotation += 0.01;
    });
});
import '../style/index.scss';
async function main()
{
    window.Module = await import("../pkg/index.js").catch(console.error);
}

main();

