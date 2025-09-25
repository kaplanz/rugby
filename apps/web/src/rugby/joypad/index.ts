import { LitElement, css, html } from "lit";
import { customElement, property } from "lit/decorators.js";

import { type GameBoy, Button as Kind } from "rugby-wasm";

import "./button";
import "./dirpad";

@customElement("gb-joypad")
export class Joypad extends LitElement {
  @property()
  emu!: GameBoy;

  private handle(event: CustomEvent) {
    const { action, button } = event.detail;
    if (action === "press") this.press(button);
    if (action === "release") this.release(button);
  }

  private static keymap: Record<string, Kind> = {
    x: Kind.A,
    z: Kind.B,
    Backspace: Kind.Select,
    Enter: Kind.Start,
    ArrowRight: Kind.Right,
    ArrowLeft: Kind.Left,
    ArrowUp: Kind.Up,
    ArrowDown: Kind.Down,
  };

  private keydown(event: KeyboardEvent) {
    // Determine button
    const btn = Joypad.keymap[event.key];
    if (btn != null) {
      // Prevent default
      event.preventDefault();
      // Forward press
      this.press(btn);
    }
  }

  private keyup(event: KeyboardEvent) {
    // Determine button
    const btn = Joypad.keymap[event.key];
    if (btn != null) {
      // Prevent default
      event.preventDefault();
      // Forward release
      this.release(btn);
    }
  }

  private press(btn: Kind) {
    this.emu.press(btn);
  }

  private release(btn: Kind) {
    this.emu.release(btn);
  }

  connectedCallback() {
    super.connectedCallback();

    // Forward key events
    window.addEventListener("keydown", this.keydown.bind(this));
    window.addEventListener("keyup", this.keyup.bind(this));
  }

  disconnectedCallback() {
    super.disconnectedCallback();

    // Remove key events
    window.removeEventListener("keydown", this.keydown.bind(this));
    window.removeEventListener("keyup", this.keyup.bind(this));
  }

  render() {
    return html`
      <div @message=${this.handle.bind(this)} class="joypad">
        <div class="top">
          <div class="dpad">
            <gb-dirpad></gb-dirpad>
          </div>
          <div class="game">
            <gb-button type="game" kind=${Kind.B}>B</gb-button>
            <gb-button type="game" kind=${Kind.A}>A</gb-button>
          </div>
        </div>
        <div class="btm">
          <div class="menu">
            <gb-button type="menu" kind=${Kind.Select}>Select</gb-button>
            <gb-button type="menu" kind=${Kind.Start}>Start</gb-button>
          </div>
        </div>
      </div>
    `;
  }

  static styles = css`
    :host {
      display: block;
    }

    .joypad {
      align-items: center;
      display: flex;
      flex-direction: column;
      gap: 6em;

      .top {
        align-items: center;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        width: 100%;

        .dpad {
          margin: 0;
        }

        .game {
          display: flex;
          gap: 3em;
          transform: rotate(-30deg);
        }
      }

      .btm {
        .menu {
          display: flex;
          gap: 2em;
          transform: translateX(-2em);

          gb-button {
            transform: rotate(-30deg);
          }
        }
      }
    }
  `;
}
