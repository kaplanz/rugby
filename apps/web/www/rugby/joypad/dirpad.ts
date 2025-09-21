import { css, html } from "lit";
import { customElement } from "lit/decorators.js";

import { Button as Kind } from "rugby-wasm";

import { Button } from "./button";

@customElement("gb-dirpad")
export class DPad extends Button {
  render() {
    const keys = [
      { id: "up", kind: Kind.Up },
      { id: "lt", kind: Kind.Left },
      { id: "rt", kind: Kind.Right },
      { id: "dn", kind: Kind.Down },
    ];
    return html`
      <div role="button" tabindex="0">
        <input id="xx" type="button"/>
        ${keys.map(
          ({ id, kind }) => html`
            <input
              id="${id}"
              type="button"
              @mousedown="${(event: MouseEvent) => this.handle(event, kind)}"
              @mouseup="${(event: MouseEvent) => this.handle(event, kind)}"
              @touchstart="${(event: TouchEvent) => this.handle(event, kind)}"
              @touchend="${(event: TouchEvent) => this.handle(event, kind)}"
            />
          `,
        )}
    </div>
    `;
  }

  static styles = css`
    :host {
      display: block;
    }

    div {
      aspect-ratio: 1 / 1;
      display: block;
      height: 100%;
      margin: 0;
      padding: 0;
      position: relative;
      width: 14em;
    }

    input[type="button"] {
      background-color: #39383a;
      border: none;
      cursor: pointer;
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
  `;
}
