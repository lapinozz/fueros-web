async function main()
{
    window.Module = await import("../pkg/index.js").catch(console.error);
}

main();

