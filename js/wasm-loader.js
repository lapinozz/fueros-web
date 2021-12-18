import * as Module from "../pkg/index.js";

function readValue(memory, ptr, type)
{
	switch(type)
	{
		case 'u32':
			return memory.getUint32(ptr, true);
		case 'u8':
			return memory.getUint8(ptr, true);
		case 'bool':
			return !!memory.getUint8(ptr, true);
		case 'i8':
			return memory.getInt8(ptr, true);
		case 'u16':
			return memory.getUint16(ptr, true);
		case 'i16':
			return memory.getInt16(ptr, true);
		case 'i32':
			return memory.getInt32(ptr, true);
		case 'u64':
			return memory.getBigUint64(ptr, true);
		case 'i64':
			return memory.getBigInt64(ptr, true);
	}
}

for (const prop of Object.getOwnPropertyNames(Module))
{
	let value = Module[prop];
	if(value.__JsEnum_Metadata)
	{
		const metadata = value.__JsEnum_Metadata();
		console.log(prop, metadata)
	}

	if(value.metadata)
	{
		const size = value.size();
		const metadata = value.metadata();
		const classDef = value;

		value = function (ptr, memory)
		{	
			//memory = memory || new DataView(shared_memory().buffer);

			//let obj = {};
			for(const meta of metadata)
			{
				this[meta.name] = readValue(memory, ptr + meta.offset, meta.ty);
			}
			//return obj;
		};

		value.prototype =
		{

		};

		console.log(prop, value, size, metadata)

		window[prop + 'Array'] = function(ptr, memory)
		{
			memory = memory || new DataView(shared_memory().buffer);

	        const values = memory.getUint32(ptr + 4 + 0, true);
	        const len = memory.getUint32(ptr + 4 + 4, true);

	        //const array = new Array(len);
	        for(let i = 0; i < len; i++)
	        {
	        	this[i] = new value(values + i * size, memory);
	        }

		}
	}

    window[prop] = value;
}
