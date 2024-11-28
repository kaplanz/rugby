<script>
import { onMount } from "svelte";

import { Cartridge, GameBoy, demo } from "/pkg/rugby.js";

import Insert from "./Insert.svelte";
import Joypad from "./Joypad.svelte";
import Screen from "./Screen.svelte";
import Stereo from "./Stereo.svelte";

const FRQ = 4194304;
const DIV = 32;

const app = $state({
  run: false,
  spd: 1.0,
});
const emu = new GameBoy();
const gui = $state({});

const tick = () => {
  for (let cycle = 0; app.run && cycle < (app.spd * FRQ) / DIV; cycle++) {
    // Emulate a single cycle
    emu.cycle();
    // Redraw on vertical sync
    if (gui.video != null && emu.vsync()) {
      gui.video.redraw(emu.frame());
    }
  }
};

function override(event) {
  event.preventDefault();
}

onMount(() => {
  // Insert a game cartridge
  emu.insert(new Cartridge(demo()));
  app.run = true;

  // Start emulation loop
  setInterval(tick, 1000 / DIV);
});
</script>

<svelte:window
  ontouchstart={override}
/>

<div class="rugby">
  <div class="top">
    <div class="frame">
      <Insert bind:this={gui.cart} bind:run={app.run} {emu} />
      <Screen bind:this={gui.video} />
    </div>
    <div class="logo">
      <span class="name">RUGBY</span>
      <span class="mark">web</span>
    </div>
  </div>
  <div class="btm">
    <div class="spacer"></div>
    <Joypad bind:this={gui.joypad} {emu} />
    <div class="audio">
      <Stereo />
    </div>
  </div>
</div>

<style>
  @import url("https://fonts.googleapis.com/css2?family=Jost:wght@600&display=swap");

  :root {
    @media screen and (max-width: 543px) {
      background-color: light-dark(#c5c0bd, #1c1a19);
    }

    @media screen and (min-width: 543px) {
      background-color: light-dark(#a2c8c8, #2f4f4f);
    }
  }

  :global :has(> .rugby) {
    align-content: center;
    user-select: none;
    -webkit-user-select: none;

    @media (hover: none) {
      touch-action: none;
      -webkit-tap-highlight-color: transparent;
      -webkit-touch-callout: none;
    }
  }

  .rugby {
    background-color: light-dark(#c5c0bd, #1c1a19);
    color: #204786;
    display: flex;
    flex-flow: column;
    height: calc(100% - 3em);
    margin: 0 auto;
    max-width: 460px;
    max-height: 760px;
    overflow: hidden;
    padding: 1em;

    @media screen and (min-width: 543px) {
      border: solid light-dark(#1c1a19, #5f5e61);
      border-radius: 1em;
      border-bottom-right-radius: 4em;
      box-shadow: 0 10px 30px -10px black;
    }

    .btm {
      display: flex;
      flex-direction: column;
      height: 100%;
      justify-content: flex-end;
    }
  }

  .audio {
    display: flex;
    justify-content: flex-end;
  }

  .frame {
    aspect-ratio: 10 / 9;
    background-color: #908d92;
    border-radius: 10px 10px 50px 10px;
    display: grid;
    padding: 5% 12.5%;

    :global * {
      grid-column: 1;
      grid-row: 1;
    }
  }

  .logo {
    color: #204786;
    font-family: "Jost";
    text-align: start;

    .name {
      font-size: x-large;
      font-style: italic;
    }

    .mark {
      font-size: x-small;
    }
  }

  .spacer {
    padding: 1em;
  }
</style>
