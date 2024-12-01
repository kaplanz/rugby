<script>
import { default as Input } from "./Button.svelte";
import DPad from "./DPad.svelte";

import { Button } from "/pkg/rugby.js";

let { emu } = $props();

function keymap(key) {
  return {
    x: Button.A,
    z: Button.B,
    " ": Button.Select,
    Enter: Button.Start,
    ArrowRight: Button.Right,
    ArrowLeft: Button.Left,
    ArrowUp: Button.Up,
    ArrowDown: Button.Down,
  }[key];
}

function keydown(event) {
  const { key } = event;
  // Determine button
  const btn = keymap(key);
  if (btn != null) {
    // Prevent default
    event.preventDefault();
    // Forward press
    press(btn);
  }
}

function keyup(event) {
  const { key } = event;
  // Determine button
  const btn = keymap(key);
  if (btn != null) {
    // Prevent default
    event.preventDefault();
    // Forward release
    release(btn);
  }
}

function handle(event) {
  const { action, button } = event.detail;
  // Handle event
  ({
    press: press,
    release: release,
  })[action](button);
}

function press(btn) {
  emu.press(btn);
}

function release(btn) {
  emu.release(btn);
}
</script>

<svelte:window onkeydown={keydown} onkeyup={keyup} />

<div class="vstack">
  <div class="hstack">
    <div class="dpad">
      <DPad on:message={handle}/>
    </div>
    <div class="game">
      <Input type="game" button={Button.B} on:message={handle}>B</Input>
      <Input type="game" button={Button.A} on:message={handle}>A</Input>
    </div>
  </div>
  <div class="menu">
    <Input type="menu" button={Button.Select} on:message={handle}>Select</Input>
    <Input type="menu" button={Button.Start} on:message={handle}>Start</Input>
  </div>
</div>

<style>
  .vstack {
    align-items: center;
    display: flex;
    flex-direction: column;
    gap: 4vh;
  }

  .hstack {
    align-items: center;
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    width: 100%;
  }

  .dpad {
    margin: 0;
  }

  .game {
    display: flex;
    gap: 2vh;
    transform: rotate(-30deg);
  }

  .menu {
    display: flex;
    gap: 1vh;

    :global div {
      transform: rotate(-30deg);
    }
  }
</style>
