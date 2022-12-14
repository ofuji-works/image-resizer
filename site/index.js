import("image-resizer").then(async (wasm) => {
    const url = "./images/sample.png";
    const res = await fetch(url);
    const blob = await res.blob();
    const resized = await resizeImageWasm(blob, 400, 400, 'png', wasm);

    const a = document.getElementById('download');
    a.href = window.URL.createObjectURL(resized);
});

async function resizeImageWasm(file, width, height, format, wasm) {
    console.log(`Original: ${file.size} Bytes`);
    const arr = new Uint8Array(await file.arrayBuffer());
  
    const result = wasm.image_resize(arr, width, height, format);

    const blob = new Blob([result]);
    console.log(`Resized: ${blob.size} Bytes`);
  
    return blob
}
