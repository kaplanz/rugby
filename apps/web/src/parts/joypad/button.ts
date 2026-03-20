import { css, html, LitElement } from "lit";
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
      border: none;
      cursor: pointer;
      font-size: inherit;
      margin: 0;
      padding: 0;
      width: 7.5em;
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
        background:
          /* hover/press overlay (animated) */
          radial-gradient(circle, #ffffff00 0%, transparent 70%),
          /* button surface */
          radial-gradient(circle at 38% 32%, #c22d6a, #7a1a44);
        border-radius: 100%;
        box-shadow:
          /* button surface highlight (light from top-left) */
          inset 2px 3px 4px #ffffff1f,
          /* button surface shadow (bottom-right depth) */
          inset -2px -3px 4px #0000008c,
          /* button drop shadow on surrounding plastic */
          0 3px 6px #00000080,
          /* cutout rim: ring of shadow just outside button */
          0 0 0 .35em light-dark(#000000a0, #0000004d),
          /* press depth shadow (animated) */
          inset 0 0 0 transparent;
        transition: background .15s, box-shadow .15s;

        &:hover {
          background:
            radial-gradient(circle, #ffffff22 0%, transparent 70%),
            radial-gradient(circle at 38% 32%, #c22d6a, #7a1a44);
        }

        &:active {
          background:
            radial-gradient(circle, #ffffff22 0%, transparent 70%),
            radial-gradient(circle at 38% 32%, #a02558, #651638);
          box-shadow:
            inset 2px 3px 4px #ffffff1f,
            inset -2px -3px 4px #0000008c,
            0 3px 6px #00000080,
            0 0 0 .35em light-dark(#000000a0, #0000004d),
            inset 0 0 1.5em #00000066;
          transition: background .06s, box-shadow .06s;
        }
      }

      label {
        font-size: 2em;
      }
    }

    div[type="menu"] {
      input {
        aspect-ratio: 11 / 2.7;
        width: 8em;
        background:
          /* hover/press overlay (animated) */
          radial-gradient(circle, #ffffff00 0%, transparent 70%),
          /* button surface */
          linear-gradient(160deg, #a8a5aa, #6e6b70);
        border-radius: 20% / 70%;
        box-shadow:
          /* plastic rim casting shadow onto button surface */
          inset 0 2px 4px #00000080,
          /* button surface sheen */
          inset 0 -1px 2px #ffffff1a,
          /* button shadow on surrounding plastic */
          0 2px 4px #00000073,
          /* tight ring: depth of the cutout rim */
          0 0 0 .35em light-dark(#000000a0, #0000004d),
          /* press depth shadow (animated) */
          inset 0 0 0 transparent;
        transition: background .15s, box-shadow .15s;

        &:hover {
          background:
            radial-gradient(circle, #ffffff22 0%, transparent 70%),
            linear-gradient(160deg, #a8a5aa, #6e6b70);
        }

        &:active {
          background:
            radial-gradient(circle, #ffffff22 0%, transparent 70%),
            linear-gradient(160deg, #8e8b90, #5a585c);
          box-shadow:
            inset 0 2px 4px #00000080,
            inset 0 -1px 2px #ffffff1a,
            0 2px 4px #00000073,
            0 0 0 .35em light-dark(#000000a0, #0000004d),
            inset 0 0 1em #00000066;
          transition: background .06s, box-shadow .06s;
        }
      }

      label {
        font-size: 1.35em;
      }
    }
  `;
}
