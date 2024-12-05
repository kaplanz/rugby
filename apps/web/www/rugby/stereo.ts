import { LitElement, css, html } from "lit";
import { customElement } from "lit/decorators.js";

@customElement("gb-stereo")
export class Stereo extends LitElement {
  render() {
    return html`
      <div>
        <span></span>
        <span></span>
        <span></span>
        <span></span>
        <span></span>
        <span></span>
      </div>
    `;
  }

  static styles = css`
    :host {
      display: block;
    }

    div {
      display: inline-flex;
      flex-direction: row;
      gap: 2.5em;
      transform: rotate(-30deg);
    }

    span {
      background-color: #5f5e61;
      border-radius: 1em;
      height: 10em;
      width: 1em;
    }
  `;
}
