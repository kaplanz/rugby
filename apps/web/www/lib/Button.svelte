<script>
import { createEventDispatcher } from "svelte";

import { Button } from "/pkg/rugby.js";

// @type {string}
export let type;
// @type {Button}
export let button;

const dispatch = createEventDispatcher();

function click(event) {
  const { type } = event;
  // Prevent defaults
  event.preventDefault();
  // Determine action
  const action = {
    mousedown: "press",
    mouseup: "release",
    touchstart: "press",
    touchend: "release",
  }[type];
  // Dispatch event
  if (action) {
    dispatch("message", {
      action,
      button,
    });
  }
}
</script>

<div {type}>
  <input
    type="button"
    id="input"
    on:mousedown={click}
    on:mouseup={click}
    on:touchstart={click}
    on:touchend={click}
  />
  <label for="input"><slot/></label>
</div>

<style>
  @import url("https://fonts.googleapis.com/css2?family=Orbitron:wght@700&display=swap");

  input {
    border: solid #1c1a19;
    margin: 0;
    padding: 0;
    user-select: none;
    width: 60px;

    &:active {
      filter: brightness(60%);
    }

    @media (prefers-color-scheme: light) {
      border: solid #1c1a19;
    }

    @media (prefers-color-scheme: dark) {
      border: solid #39383a;
    }
  }

  label {
    color: #204786;
    font-family: "Orbitron";
  }

  div {
    align-items: center;
    display: flex;
    flex-direction: column;
    text-transform: uppercase;
  }

  div[type="game"] {
    & input {
      aspect-ratio: 1 / 1;
      background-color: #9a2257;
      border-radius: 100%;
    }
  }

  div[type="menu"] {
    & input {
      aspect-ratio: 5 / 2;
      background-color: #908d92;
      border-radius: 20% / 50%;
    }

    & label {
      font-size: x-small;
    }
  }
</style>
