import fs from "fs";

async function createBase64UriForWasm(filePath) {
  const base64 = await fs.promises.readFile(filePath, "base64");
  return "data:application/wasm;base64," + base64;
}

async function createUint8ArrayForWasm(filePath) {
  const buff = await fs.promises.readFile(filePath);
  return new Uint8Array(buff);
}

async function bufferToBytes(buff) {
  let bytes;
  if (typeof Buffer === "function" && typeof Buffer.from === "function") {
    console.log("Buffer");
    bytes = Buffer.from(buff, "base64");
  } else if (typeof atob === "function") {
    console.log("atob");
    const binaryString = atob(buff);
    bytes = new Uint8Array(binaryString.length);
    for (let i = 0; i < binaryString.length; i++) {
      bytes[i] = binaryString.charCodeAt(i);
    }
  }
  return bytes;
}

const buff = await createBase64UriForWasm("./wasm/index_bg.wasm");
const bytes = await bufferToBytes(buff);

console.log(bytes.length);

const u8 = await createUint8ArrayForWasm("./wasm/index_bg.wasm");

const u8Buffer = new Uint8Array(u8);
console.log(u8Buffer.length);
console.log(u8.length);

console.log(u8Buffer[0]);
console.log(u8[0]);

console.log(u8Buffer[1]);
console.log(u8[1]);