<script>
import { onMount } from "svelte";

const SIZE = { wd: 160, ht: 144 };

// @type {HTMLCanvasElement}
let canvas;
// @type {CanvasRenderingContext2D}
let ctx;

onMount(() => {
  ctx = canvas.getContext("2d");
});

export function redraw(frame) {
  // Create an image to draw to the canvas
  const image = ctx.createImageData(160, 144);
  // Draw extracted frame data into the canvas
  for (let idx = 0; idx < 160 * 144; idx++) {
    image.data[4 * idx + 0] = 0;
    image.data[4 * idx + 1] = 0;
    image.data[4 * idx + 2] = 0;
    image.data[4 * idx + 3] = 32 * frame[idx];
  }
  // Display the image in the canvas
  ctx.putImageData(image, 0, 0);
}
</script>

<canvas bind:this={canvas} width={SIZE.wd} height={SIZE.ht} />

<style>
  canvas {
    background-color: #8ca05a;
    image-rendering: pixelated;
    width: 100%;
    max-height: 100%;
  }
</style>
