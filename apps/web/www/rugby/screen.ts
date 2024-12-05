import { LitElement, css, html } from "lit";
import { customElement } from "lit/decorators.js";

const SIZE = { wd: 160, ht: 144 };

@customElement("gb-screen")
export class Screen extends LitElement {
  private ctx!: CanvasRenderingContext2D;

  firstUpdated() {
    // biome-ignore lint: lint/style/noNonNullAssertion
    this.ctx = this.shadowRoot?.querySelector("canvas")?.getContext("2d")!;
  }

  clear() {
    this.redraw(new Uint8Array());
  }

  redraw(frame: Uint8Array) {
    // Create an image to draw to the canvas
    const image = this.ctx.createImageData(160, 144);

    // Draw extracted frame data into the canvas
    for (let idx = 0; idx < 160 * 144; idx++) {
      // Don't use any color channels
      image.data[4 * idx + 0] = 0;
      image.data[4 * idx + 1] = 0;
      image.data[4 * idx + 2] = 0;
      // Instead, just use alpha channel
      image.data[4 * idx + 3] = 32 * (frame[idx] ?? 0);
    }

    // Display the image in the canvas
    this.ctx.putImageData(image, 0, 0);
    this.requestUpdate(); // redraw canvas
  }

  render() {
    return html`
      <canvas width="${SIZE.wd}" height="${SIZE.ht}"></canvas>
    `;
  }

  static styles = css`
    :host {
      display: block;
    }

    canvas {
      aspect-ratio: 10 / 9;
      background: linear-gradient(to bottom, #3c4719, #505b20);
      border: .4em solid black;
      image-rendering: pixelated;
      max-height: 100%;
      width: 100%;
    }
  `;
}
