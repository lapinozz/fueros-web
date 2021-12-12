import "./components.scss";
import { menu } from "./menu";

async function main() {
    window.Module = await import("../pkg/index.js").catch(console.error);
    menu();
}

main();
