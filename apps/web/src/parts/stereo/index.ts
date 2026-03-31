import { css, html, LitElement } from "lit";
import { customElement } from "lit/decorators.js";

import type { Chiptune } from "rugby-wasm";

import worklet from "./worklet.js?raw";

export const SAMPLE = 44_100;

@customElement("gb-stereo")
export class Stereo extends LitElement {
  /** Audio context. */
  private ctx = new AudioContext({ sampleRate: SAMPLE });

  /** Audio worklet node. */
  private node?: AudioWorkletNode;

  get playing() {
    return this.ctx.state === "running";
  }

  async firstUpdated() {
    const blob = new Blob([worklet], { type: "application/javascript" });
    const url = URL.createObjectURL(blob);
    await this.ctx.audioWorklet.addModule(url);
    URL.revokeObjectURL(url);
    this.node = new AudioWorkletNode(this.ctx, "chiptune", {
      outputChannelCount: [2],
    });
    this.node.connect(this.ctx.destination);
    this.node.port.onmessage = (event) => {
      console.log(event.data);
    };
  }

  /**
   * Toggles between suspended and running.
   */
  toggle(): Promise<void> {
    if (this.ctx.state !== "running") {
      return this.ctx.resume();
    } else {
      return this.ctx.suspend();
    }
  }

  /**
   * Plays the given chiptune sample.
   */
  async play(sample: Chiptune) {
    const { vol, ch1, ch2, ch3, ch4 } = sample;
    const out = [ch1, ch2, ch3, ch4].reduce(
      (acc, itm) => {
        acc.lt += (itm.lt * vol.lt) / 4;
        acc.rt += (itm.rt * vol.rt) / 4;
        return acc;
      },
      { lt: 0, rt: 0 },
    );
    this.node?.port.postMessage(out);
  }

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
      background: light-dark(
        radial-gradient(ellipse 80% 30% at 50% 15%, #3c3a3e, #0c0b0d),
        radial-gradient(ellipse 80% 30% at 50% 15%, #252428, #0c0b0d)
      );
      border-radius: 1em;
      box-shadow:
        inset 0 3px 4px light-dark(#000000e6, #000000b3),
        inset 0 -3px 4px light-dark(#000000e6, #000000b3),
        inset 2px 0 3px light-dark(#00000099, #00000066),
        inset -2px 0 3px light-dark(#00000099, #00000066),
        0 1px 0 light-dark(#ffffff1a, #ffffff0f);
      height: 10em;
      width: 1em;
    }
  `;
}
