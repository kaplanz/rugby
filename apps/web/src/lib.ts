import "@/assets/fonts/index.css";

import noise from "@/assets/img/noise.png";

import { setBasePath } from "@shoelace-style/shoelace/dist/utilities/base-path.js";
import { css, html, LitElement, unsafeCSS } from "lit";
import { customElement, query, state } from "lit/decorators.js";

import { Settings, Volume2, VolumeX } from "lucide";

import { Cartridge, demo } from "rugby-wasm";

setBasePath(
  "https://cdn.jsdelivr.net/npm/@shoelace-style/shoelace@2.19.0/cdn/",
);

import type { Dialog } from "./parts/dialog";
import { App } from "./app";
import type { Screen } from "./parts/screen";
import type { Stereo } from "./parts/stereo";

import "./parts/dialog";
import "./parts/joypad";
import "./parts/screen";
import "./parts/stereo";
import "./parts/switch";

import { icon } from "./util";

/**
 * Game Boy emulator.
 */
@customElement("game-boy")
export class GameBoy extends LitElement {
  /** Application state. */
  @state()
  private app = new App();

  /** Audio speaker output. */
  @query("gb-stereo")
  private apu!: Stereo;

  /** Graphical display model. */
  @query("gb-screen")
  private lcd!: Screen;

  /** Configuration interface. */
  @query("gb-dialog")
  private cfg!: Dialog;

  /**
   * Opens menu dialog.
   */
  menu() {
    this.cfg.show();
  }

  private toggleAudio = async () => {
    await this.apu.toggle();
    this.requestUpdate();
  };

  constructor() {
    super();

    // Request update on application state change.
    this.app.draw = () => {
      this.requestUpdate();
    };
  }

  firstUpdated() {
    // Trigger a re-render to initialize
    this.requestUpdate();
    // Connect components to application
    this.app.gui.apu = this.apu;
    this.app.gui.lcd = this.lcd;
  }

  connectedCallback() {
    super.connectedCallback();

    // Insert a game cartridge
    this.app.emu.insert(new Cartridge(demo()));
    // Enable emulation.
    this.app.start();
  }

  disconnectedCallback() {
    super.disconnectedCallback();

    // Clear emulation loop
    this.app.stop();
  }

  render() {
    return html`
      <gb-dialog .app=${this.app}></gb-dialog>
      <main>
        <gb-switch .app=${this.app} .lcd=${this.lcd}></gb-switch>
        <div id="controls">
          <button id="menu" @click=${() => this.cfg.show()}>${icon(Settings)}</button>
          <button id="audio" @click=${this.toggleAudio}>${this.apu?.playing ? icon(Volume2) : icon(VolumeX)}</button>
        </div>
        <div class="top">
          <div class="shape"></div>
          <div class="frame">
            <div class="label">
              <span>Dot Matrix with Stereo Sound</span>
            </div>
            <div class="power">
              <div class="led" ?power="${this.app.ctx.run}"></div>
              <div class="txt">battery</div>
            </div>
            <gb-screen></gb-screen>
          </div>
          <div class="logo">
            <span class="brand">Rugby</span>
            <span class="model">Game Boy</span>
            <span class="tmark">web</span>
          </div>
        </div>
        <div class="btm">
          <gb-joypad .emu=${this.app.emu}></gb-joypad>
          <gb-stereo></gb-stereo>
        </div>
      </main>
    `;
  }

  static styles = css`
    :host {
      container-type: size;
      display: block;

      aspect-ratio: 90 / 148;
      inline-size: 100%;
      max-inline-size: 450px;

      user-select: none;
      -webkit-user-select: none;

      @media (hover: none) {
        -webkit-tap-highlight-color: transparent;
        -webkit-touch-callout: none;
      }
    }

    main {
      display: flex;
      flex-flow: column;
      justify-content: space-between;
      position: relative;

      block-size: 100cqb;
      inline-size: 100cqi;

      padding: 4cqb;

      border-color: light-dark(#1c1a19, #5f5e61);
      border-radius: 2.5cqb;
      border-bottom-right-radius: 15cqb;
      border-style: solid;
      border-width: .5cqb;
      box-shadow: 0 10px 30px -10px black;
      box-sizing: border-box;
      overflow: hidden;

      background-color: light-dark(#c5c0bd, #1c1a19);
      background-image: url(${unsafeCSS(noise)});
      background-blend-mode: soft-light;

      color: #204786;
      font-family: "Cabin";
      font-size: min(7.40px, 1cqb);

      gb-switch {
        left: 6cqb;
        position: absolute;
        top: 1.25cqb;
        z-index: 1;
      }

      #controls {
        position: absolute;
        bottom: 0;
        left: 0;
        z-index: 1;

        display: flex;
        gap: .5cqb;
        margin: 1cqb;

        button {
          padding: 0;
          border: 1px solid color-mix(in srgb, currentColor 18%, transparent);
          background: color-mix(in srgb, currentColor 6%, transparent);
          color: light-dark(#204786, #7aa0dd);
          cursor: pointer;
          font-size: 3cqb;
          transition: background .3s, border-color .3s;

          width: 5cqb;
          height: 5cqb;

          border-radius: 50%;
          display: grid;
          place-items: center;

          &:hover {
            background: color-mix(in srgb, currentColor 20%, transparent);
            border-color: color-mix(in srgb, currentColor 70%, transparent);
          }

          &:active {
            background: color-mix(in srgb, currentColor 40%, transparent);
            border-color: color-mix(in srgb, currentColor 90%, transparent);
          }
        }

      }

      .top {
        position: relative;

        .shape {
          background-color: light-dark(#928f8d, #5f5e61);
          height: .75cqb;
          left: -4cqb;
          position: absolute;
          top: 0;
          width: calc(100% + 8cqb);

          &:before,
          &:after {
            background-color: inherit;
            content: "";
            display: block;
            height: 4cqb;
            position: absolute;
            top: -4cqb;
            width: .75cqb;
          }

          &:before {
            left: 4cqb;
          }

          &:after {
            right: 4cqb;
          }
        }

        .frame {
          background-color: #585862;
          border: .2cqb solid light-dark(#1c1a19, #5f5e61);
          border-radius: 2cqb;
          border-bottom-right-radius: 8cqb;
          color: #c5c0bd;
          font-weight: 100;
          letter-spacing: .15cqb;
          margin-top: 3cqb;
          padding: 7.5% 18%;
          position: relative;
          text-transform: uppercase;

          .label {
            display: flex;
            gap: .5cqb;
            left: 0;
            margin: 0 2cqb;
            place-content: space-between;
            position: absolute;
            right: 0;
            top: 1.25cqb;

            span {
              font-size: 1.4cqb;
            }

            &:before,
            &:after {
              border-bottom: .45cqb solid #204786;
              border-top: .45cqb solid #9a2257;
              content: "";
              display: inline-block;
              height: .6cqb;
            }

            &:before {
              left: 0;
              width: 29%;
            }

            &:after {
              right: 0;
              width: 12%;
            }
          }

          .power {
            display: flex;
            flex-direction: column;
            font-size: .9cqb;
            gap: 1.6cqb;
            position: absolute;
            left: 2cqb;
            top: 36%;

            .txt {
              font-size: 1.4cqb;
            }

            .led {
              aspect-ratio: 1;
              background-color: black;
              border: .25cqb solid black;
              border-radius: 50%;
              box-sizing: border-box;
              display: block;
              left: 1cqb;
              position: relative;
              width: 1.8cqb;

              &[power] {
                background-color: red;
                box-shadow: 0 0 3cqb .5cqb red;
                filter: blur(.2cqb);
              }
            }
          }
        }

        .logo {
          color: #204786;
          text-align: start;

          .brand {
            font-family: "Pretendo";
            font-size: 2.5cqb;
          }

          .model {
            font-size: 4.5cqb;
            font-style: italic;
            font-weight: 600;
            text-transform: uppercase;
          }

          .tmark {
            font-weight: 900;
            text-transform: uppercase;
          }
        }
      }

      .btm {
        display: flex;
        flex-direction: column;
        position: relative;

        gb-joypad {
          margin-bottom: 9cqb;
        }

        gb-stereo {
          bottom: 2%;
          left: 67%;
          position: absolute;
        }
      }
    }
  `;
}
