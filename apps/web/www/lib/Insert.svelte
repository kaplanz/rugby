<script>
import { onMount } from "svelte";

import { Cartridge, GameBoy } from "/pkg/rugby.js";

// @type {boolean}
export let run;
// @type {GameBoy}
export let emu;

// @type {FileList}
let files;

export function insert(data) {
  // Stop emulation
  run = false;
  // Construct a cartridge from the data
  const cart = new Cartridge(data);
  // Insert the cartridge
  emu.insert(cart);
  console.log("inserted game cartridge");
  // Reset the emulator
  emu.reset();
  // Resume emulation
  run = true;
}

export function upload() {
  // Read the uploaded file
  const reader = new FileReader();
  reader.readAsArrayBuffer(files[0]);
  // When ready, insert into the console
  reader.onload = async (event) => {
    const data = new Uint8Array(event.target.result);
    insert(data);
  };
}
</script>

<label on:change={upload}>
  <input type="file" select=".gb,.gbc" bind:files />
</label>

<style>
  input {
    display: none;
  }

  label:has(> input) {
    z-index: 1;
  }
</style>
