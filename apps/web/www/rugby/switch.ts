import { LitElement, css, html } from "lit";
import { customElement, property, query } from "lit/decorators.js";

import { Application } from "../app";
import { Screen } from "./screen";

@customElement("gb-switch")
export class Stereo extends LitElement {
  @property()
  app!: Application;

  @property()
  lcd!: Screen;

  @query("input")
  private box!: HTMLInputElement;

  enable() {
    // Reset when disabled
    if (!this.box.checked) {
      // Clear the display
      this.lcd.clear();
      // Reset the emulator
      this.app.emu.reset();
    }
    // Play/stop emulation
    this.app.play(this.box.checked);
  }

  render() {
    return html`
      <input
        id="switch"
        type="checkbox"
        @click=${this.enable.bind(this)}
        ?checked=${this.app.cfg.run}
      ></input>
      <label for="switch">
        <span>&#x25c0;&#xfe0e; off &bullet; on &#x25b6;&#xfe0e;</span>
      </label>
      <div>
        <span></span>
        <span></span>
        <span></span>
      </div>
    `;
  }

  static styles = css`
    :host {
      display: block;
      position: relative;
    }

    input {
      display: none;
      margin: 0;
    }

    label {
      background-color: light-dark(#928f8d, #5f5e61);
      border-radius: 2em;
      color: light-dark(#c5c0bd, #1c1a19);
      cursor: pointer;
      font-size: 1.5em;
      font-weight: 900;
      padding: .25em .75em;
      text-transform: uppercase;
    }

    div {
      display: flex;
      gap: .5em;
      position: absolute;
      top: -1.25em;
      left: 1em;
      width: 1.5em;

      span {
        background-color: light-dark(#928f8d, #5f5e61);
        display: block;
        height: 1em;
        padding: 0 .4em;
      }
    }
  `;
}
