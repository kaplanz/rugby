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

    // Start emulation loop
    this.app.tick(1.0);
    // Insert a game cartridge
    this.app.emu.insert(new Cartridge(demo()));
    // Enable emulation.
    this.app.play();
  }

  disconnectedCallback() {
    super.disconnectedCallback();

    // Clear emulation loop
    this.app.tick();
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
              <div class="led" ?power="${this.app.cfg.run}"></div>
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
      display: block;

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

      aspect-ratio: 90 / 148;
      height: min(740px, 148dvw / .9, 100dvh);
      max-width: min(450px, 100dvw);

      margin: auto;
      padding: 4em;

      border-color: light-dark(#1c1a19, #5f5e61);
      border-radius: 2.5em;
      border-bottom-right-radius: 15em;
      border-style: solid;
      border-width: .5em;
      box-shadow: 0 10px 30px -10px black;
      box-sizing: border-box;
      overflow: hidden;

      background-color: light-dark(#c5c0bd, #1c1a19);
      background-image: url(${unsafeCSS(noise)});
      background-blend-mode: soft-light;

      color: #204786;
      font-family: "Cabin";
      font-size: min(7.40px, 148dvw / 90, 1dvh);

      gb-switch {
        left: 6em;
        position: absolute;
        top: 1.25em;
        z-index: 1;
      }

      #controls {
        position: absolute;
        bottom: 0;
        left: 0;
        z-index: 1;

        display: flex;
        gap: .5em;
        margin: .5em;

        button {
          padding: 0;
          border: 1px solid color-mix(in srgb, currentColor 18%, transparent);
          background: color-mix(in srgb, currentColor 6%, transparent);
          color: light-dark(#204786, #7aa0dd);
          cursor: pointer;
          font-size: clamp(14px, 2dvh, 22px);
          transition: background .3s, border-color .3s;

          width: 1.75em;
          height: 1.75em;

          @media (hover: none) {
            font-size: clamp(20px, 2dvh, 28px);
          }
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
          height: .75em;
          left: -4em;
          position: absolute;
          top: 0;
          width: calc(100% + 8em);

          &:before,
          &:after {
            background-color: inherit;
            content: "";
            display: block;
            height: 4em;
            position: absolute;
            top: -4em;
            width: .75em;
          }

          &:before {
            left: 4em;
          }

          &:after {
            right: 4em;
          }
        }

        .frame {
          background-color: #585862;
          border: .2em solid light-dark(#1c1a19, #5f5e61);
          border-radius: 2em;
          border-bottom-right-radius: 8em;
          color: #c5c0bd;
          font-weight: 100;
          letter-spacing: .15em;
          margin-top: 3em;
          padding: 7.5% 18%;
          position: relative;
          text-transform: uppercase;

          .label {
            display: flex;
            gap: .5em;
            left: 0;
            margin: 0 2em;
            place-content: space-between;
            position: absolute;
            right: 0;
            top: 1.25em;

            span {
              font-size: 1.4em;
            }

            &:before,
            &:after {
              border-bottom: .45em solid #204786;
              border-top: .45em solid #9a2257;
              content: "";
              display: inline-block;
              height: .6em;
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
            font-size: .9em;
            gap: 1.6em;
            position: absolute;
            left: 2em;
            top: 36%;

            .txt {
              font-size: 1.4em;
            }

            .led {
              aspect-ratio: 1;
              background-color: black;
              border: .25em solid black;
              border-radius: 50%;
              box-sizing: border-box;
              display: block;
              left: 1em;
              position: relative;
              width: 1.8em;

              &[power] {
                background-color: red;
                box-shadow: 0 0 3em .5em red;
                filter: blur(.2em);
              }
            }
          }
        }

        .logo {
          color: #204786;
          text-align: start;

          .brand {
            font-family: "Pretendo";
            font-size: 2.5em;
          }

          .model {
            font-size: 4.5em;
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
          margin-bottom: 9em;
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
