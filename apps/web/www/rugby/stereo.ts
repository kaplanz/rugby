import { LitElement, css, html } from "lit";
import { customElement } from "lit/decorators.js";

import type { Chiptune } from "rugby-web";

export const SAMPLE = 96_000;

@customElement("gb-stereo")
export class Stereo extends LitElement {
  /** Audio context. */
  private ctx = new AudioContext({ sampleRate: SAMPLE });

  /** Audio worklet node. */
  private node?: AudioWorkletNode;

  async firstUpdated() {
    // Load the audio worklet
    await this.ctx.audioWorklet.addModule("/chiptune.js");
    // Create an audio worklet node processor
    this.node = new AudioWorkletNode(this.ctx, "chiptune", {
      outputChannelCount: [2],
    });
    // Connext the worklet as an audio output
    this.node.connect(this.ctx.destination);
    // Forward worklet messages
    this.node.port.onmessage = (event) => {
      console.log(event.data);
    };
  }

  /**
   * Ensures that the `AudioContext` is resumed after a user gesture.
   */
  private async resume() {
    // Resume audio context if needed
    if (this.ctx.state === "suspended") {
      await this.ctx.resume();
    }
  }

  /**
   * Plays the given chiptune sample.
   */
  async play(sample: Chiptune) {
    // Mix the chiptune into a sample
    const { vol, ch1, ch2, ch3, ch4 } = sample;
    const out = [ch1, ch2, ch3, ch4].reduce(
      (acc, itm) => {
        acc.lt += (itm.lt * vol.lt) / 4;
        acc.rt += (itm.rt * vol.rt) / 4;
        return acc;
      },
      { lt: 0, rt: 0 },
    );

    // Forward sample to the processor
    this.node?.port.postMessage(out);
  }

  render() {
    return html`
      <div @click=${this.resume}>
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
