import { css, html, svg } from "lit";
import { customElement } from "lit/decorators.js";

import { Button as Kind } from "rugby-wasm";

import { Button } from "./button";

// Cross shape with rounded outer corners (3×3 viewBox, arm width = 1 unit, r = 0.15)
const cross = svg`<path d="M1.15 0L1.85 0Q2 0 2 .15L2 1L2.85 1Q3 1 3 1.15L3 1.85Q3 2 2.85 2L2 2L2 2.85Q2 3 1.85 3L1.15 3Q1 3 1 2.85L1 2L.15 2Q0 2 0 1.85L0 1.15Q0 1 .15 1L1 1L1 .15Q1 0 1.15 0Z" fill="#3a383c" stroke="#00000066" stroke-width="0.12" stroke-linejoin="round" paint-order="stroke fill"/>`;

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
        <svg viewBox="-0.2 -0.2 3.4 3.4" aria-hidden="true">
          <defs>
            <linearGradient id="hi" x1="0" y1="0" x2="1" y2="1">
              <stop offset="0%" stop-color="#ffffff33"/>
              <stop offset="40%" stop-color="#00000000"/>
            </linearGradient>
            <linearGradient id="sh" x1="1" y1="1" x2="0" y2="0">
              <stop offset="0%" stop-color="#000000b0"/>
              <stop offset="45%" stop-color="#00000000"/>
            </linearGradient>
            <clipPath id="c">
              <path d="M1.15 0L1.85 0Q2 0 2 .15L2 1L2.85 1Q3 1 3 1.15L3 1.85Q3 2 2.85 2L2 2L2 2.85Q2 3 1.85 3L1.15 3Q1 3 1 2.85L1 2L.15 2Q0 2 0 1.85L0 1.15Q0 1 .15 1L1 1L1 .15Q1 0 1.15 0Z"/>
            </clipPath>
          </defs>
          ${cross}
          <rect x="0" y="0" width="3" height="3" fill="url(#hi)" clip-path="url(#c)"/>
          <rect x="0" y="0" width="3" height="3" fill="url(#sh)" clip-path="url(#c)"/>
          <circle cx="1.5" cy="1.5" r="0.38" fill="#252426"/>
        </svg>
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
      width: 16em;
    }

    svg {
      filter: drop-shadow(0 4px 4px #000000bb);
      height: 100%;
      left: 0;
      pointer-events: none;
      position: absolute;
      top: 0;
      width: 100%;
    }

    input[type="button"] {
      background: radial-gradient(circle, #ffffff22 0%, transparent 70%);
      box-shadow: inset 0 0 0 0 transparent;
      border: none;
      cursor: pointer;
      height: 29.4%;
      margin: 0;
      opacity: 0;
      padding: 0;
      position: absolute;
      transition: opacity .2s, box-shadow .1s, background .15s;
      width: 29.4%;

      &:hover {
        opacity: 1;
      }

      &:active {
        background: radial-gradient(circle, #ffffff22 0%, transparent 70%);
        opacity: 1;
        transition: opacity 0s, box-shadow .06s, background 0s;
      }
    }

    #up:active { box-shadow: inset 0  1em 1em 0 rgba(0, 0, 0, 0.4); }
    #dn:active { box-shadow: inset 0 -1em 1em 0 rgba(0, 0, 0, 0.4); }
    #lt:active { box-shadow: inset  1em 0 1em 0 rgba(0, 0, 0, 0.4); }
    #rt:active { box-shadow: inset -1em 0 1em 0 rgba(0, 0, 0, 0.4); }

    #up {
      left: 35.3%;
      top: 5.9%;
    }

    #dn {
      left: 35.3%;
      top: 64.7%;
    }

    #lt {
      left: 5.9%;
      top: 35.3%;
    }

    #rt {
      left: 64.7%;
      top: 35.3%;
    }

    #xx {
      cursor: default;
      left: 35.3%;
      pointer-events: none;
      top: 35.3%;
    }
  `;
}
