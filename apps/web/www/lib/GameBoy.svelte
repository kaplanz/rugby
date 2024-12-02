<script>
import { onMount } from "svelte";

import { Cartridge, GameBoy, demo } from "/pkg/rugby.js";

import { default as GamePak } from "./cart/Cartridge.svelte";
import Joypad from "./joypad/Joypad.svelte";
import Screen from "./screen/Screen.svelte";
import Speaker from "./audio/Speaker.svelte";

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
      <GamePak bind:this={gui.cart} bind:run={app.run} {emu} />
      <Screen bind:this={gui.video} />
    </div>
    <div class="logo">
      <span class="name">RUGBY</span>
      <span class="mark">web</span>
    </div>
  </div>
  <div class="btm">
    <Joypad bind:this={gui.joypad} {emu} />
    <div class="audio">
      <Speaker />
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
    aspect-ratio: 90 / 148;
    background-color: light-dark(#c5c0bd, #1c1a19);
    box-sizing: border-box;
    color: #204786;
    display: flex;
    flex-flow: column;
    height: calc(100% - 3em);
    margin: 0 auto;
    max-height: 740px;
    max-width: 100%;
    overflow: hidden;
    padding: 2vh;

    @media screen and (min-width: 543px) {
      border: .5vh solid light-dark(#1c1a19, #5f5e61);
      border-radius: 4.44% / 2.7%;
      border-bottom-right-radius: 17.76% 10.8%;
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
    border-radius: 4.5% / 5%;
    border-bottom-right-radius: 13.5% 15%;
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
      font-size: 3vh;
      font-style: italic;
    }

    .mark {
      font-size: 1.5vh;
    }
  }
</style>
