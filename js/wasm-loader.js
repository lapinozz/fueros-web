import * as Module from "../pkg/index.js";

for (const prop of Object.getOwnPropertyNames(Module))
{
	const value = Module[prop];
	if(value.__JsEnum_Metadata)
	{
		const metadata = value.__JsEnum_Metadata();
		console.log(prop, metadata)
	}

    window[prop] = Module[prop];
}
