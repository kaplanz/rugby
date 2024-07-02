<script>
import { createEventDispatcher } from "svelte";

import { Button } from "/pkg/rugby.js";

const dispatch = createEventDispatcher();

function click(event) {
  const { target, type } = event;
  // Prevent defaults
  event.preventDefault();
  // Determine action
  const action = {
    mousedown: "press",
    mouseup: "release",
    touchstart: "press",
    touchend: "release",
  }[type];
  // Determine button
  const button = {
    up: Button.Up,
    dn: Button.Down,
    lt: Button.Left,
    rt: Button.Right,
  }[target.id];
  // Dispatch event
  if (action && button) {
    dispatch("message", {
      action,
      button,
    });
  }
}
</script>

<div
  role="button"
  tabindex={0}
  on:mousedown={click}
  on:mouseup={click}
  on:touchstart={click}
  on:touchend={click}
>
  <input type="button" id="up"/>
  <input type="button" id="lt"/>
  <input type="button" id="xx"/>
  <input type="button" id="rt"/>
  <input type="button" id="dn"/>
</div>

<style>
  div {
    aspect-ratio: 1 / 1;
    display: block;
    height: 100%;
    margin: 0;
    padding: 0;
    position: relative;
    width: 120px;
  }

  input[type="button"] {
    background-color: #333;
    border: none;
    height: 33%;
    margin: 0;
    padding: 0;
    position: absolute;
    width: 33%;

    &:active {
      filter: brightness(60%);
    }
  }

  #up {
    border-radius: 15% 15% 0 0;
    left: 33%;
    top: 0;
  }

  #dn {
    border-radius: 0 0 15% 15%;
    left: 33%;
    top: 66%;
  }

  #lt {
    border-radius: 15% 0 0 15%;
    left: 0;
    top: 33%;
  }

  #rt {
    border-radius: 0 15% 15% 0;
    left: 66%;
    top: 33%;
  }

  #xx {
    border-radius: 0;
    left: 33%;
    top: 33%;

    &:active {
      filter: none !important;
    }
  }
</style>
