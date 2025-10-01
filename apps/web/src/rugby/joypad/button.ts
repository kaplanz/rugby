import { LitElement, css, html } from "lit";
import { customElement, property } from "lit/decorators.js";

import type { Button as Kind } from "rugby-wasm";

@customElement("gb-button")
export class Button extends LitElement {
  @property()
  type = "game";
  @property()
  kind!: Kind;

  protected handle(event: MouseEvent | TouchEvent, button = this.kind) {
    // Prevent defaults
    event.preventDefault();

    // Determine action
    const action = {
      mousedown: "press",
      mouseup: "release",
      touchstart: "press",
      touchend: "release",
    }[event.type];

    // Dispatch event
    if (action) {
      this.dispatchEvent(
        new CustomEvent("message", {
          detail: { action, button },
          bubbles: true,
          composed: true,
        }),
      );
    }
  }

  render() {
    return html`
      <div type="${this.type}">
        <input
          type="button"
          @mousedown="${this.handle}"
          @mouseup="${this.handle}"
          @touchstart="${this.handle}"
          @touchend="${this.handle}"
        />
        <label/><slot></slot></label>
      </div>
    `;
  }

  static styles = css`
    :host {
      color: #204786;
      display: block;
      font-family: "Orbitron";
      font-weight: 900;
    }

    input {
      border: .4em solid light-dark(#1c1a19, #39383a);
      cursor: pointer;
      font-size: inherit;
      margin: 0;
      padding: 0;
      width: 7.5em;

      &:active {
        filter: brightness(60%);
      }
    }

    div {
      align-items: center;
      display: flex;
      flex-direction: column;
      gap: .75em;
      text-transform: uppercase;
    }

    div[type="game"] {
      input {
        aspect-ratio: 1 / 1;
        background-color: #9a2257;
        border-radius: 100%;
      }

      label {
        font-size: 2em;
      }
    }

    div[type="menu"] {
      input {
        aspect-ratio: 7 / 2;
        background-color: #908d92;
        border-radius: 20% / 70%;
      }

      label {
        font-size: 1.6em;
      }
    }
  `;
}
