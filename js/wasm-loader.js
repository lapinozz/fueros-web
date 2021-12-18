import * as Module from "../pkg/index.js";

export function readValue(memory, ptr, type) {
    switch (type) {
        case "u32":
            return memory.getUint32(ptr, true);
        case "u8":
            return memory.getUint8(ptr, true);
        case "bool":
            return !!memory.getUint8(ptr, true);
        case "i8":
            return memory.getInt8(ptr, true);
        case "u16":
            return memory.getUint16(ptr, true);
        case "i16":
            return memory.getInt16(ptr, true);
        case "i32":
            return memory.getInt32(ptr, true);
        case "u64":
            return memory.getBigUint64(ptr, true);
        case "i64":
            return memory.getBigInt64(ptr, true);
    }
}

export function readStruct(structName, memory, ptr, index) {
    const metadata = window[structName + "_metadata"]();
    const size = window[structName + "_size"]();
    let out = {};
    for (const field of metadata) {
        out[field.name] = readValue(
            memory,
            ptr + index * size + field.offset,
            field.ty
        );
    }
    return out;
}

for (const prop of Object.getOwnPropertyNames(Module)) {
    let value = Module[prop];
    window[prop] = value;
}
