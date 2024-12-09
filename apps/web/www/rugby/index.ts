import { LitElement, css, html } from "lit";
import { customElement, query, state } from "lit/decorators.js";

import { Cartridge, demo } from "rugby-web";

import { Screen } from "./screen";
import { Application } from "../app";

import "./joypad";
import "./loader";
import "./screen";
import "./stereo";
import "./switch";

/**
 * Clock frequency.
 *
 * @remarks
 * Frequency of the on-board quartz crystal used for timekeeping.
 */
const FRQ = 4194304;
/**
 * Clock divider.
 *
 * @remarks
 * As an optimization, we use a clock divider and simulate a batch of `DIV`
 * cycles at a frequency of `FRQ / DIV`.
 */
const DIV = 32;

/**
 * Game Boy emulator.
 */
@customElement("game-boy")
export class GameBoy extends LitElement {
  /** Application state. */
  @state()
  private app = new Application();

  /** Clock thread handle. */
  private tid?: number;

  /** Graphical display model. */
  @query("gb-screen")
  private lcd!: Screen;

  /**
   * Emulates a single cycle.
   */
  private cycle() {
    // No-op if paused
    if (!this.app.cfg.run) return;

    // Iterate pending cycles
    for (let tick = 0; tick < (this.app.cfg.spd * FRQ) / DIV; tick++) {
      // Emulate a single cycle
      this.app.emu.cycle();
      // Redraw on vertical sync
      if (this.lcd && this.app.emu.vsync()) {
        // Get next frame
        const frame = this.app.emu.frame();
        // Redraw display
        this.lcd.redraw(frame);
      }
    }
  }

  constructor() {
    super();

    // Request update on application state change.
    this.app.callback = () => {
      this.requestUpdate();
    }
  }

  firstUpdated() {
    // Trigger a re-render to propagete `lcd`
    this.requestUpdate();
  }

  connectedCallback() {
    super.connectedCallback();

    // Start emulation loop
    this.tid = setInterval(this.cycle.bind(this), 1000 / DIV);
    // Insert a game cartridge
    this.app.emu.insert(new Cartridge(demo()));
    // Enable emulation.
    this.app.play(true);
  }

  disconnectedCallback() {
    super.disconnectedCallback();

    // Clear emulation loop
    clearInterval(this.tid);
  }

  render() {
    return html`
      <main>
        <gb-switch .app=${this.app} .lcd=${this.lcd}></gb-switch>
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
      margin: auto;

      aspect-ratio: 90 / 148;
      height: min(740px, 148vw / .9, 100vh);
      max-width: min(450px, 100vw);

      user-select: none;
      -webkit-user-select: none;

      @media (hover: none) {
        touch-action: none;
        -webkit-tap-highlight-color: transparent;
        -webkit-touch-callout: none;
      }
    }

    main {
      background-color: light-dark(#c5c0bd, #1c1a19);
      box-sizing: border-box;
      color: #204786;
      display: flex;
      flex-flow: column;
      font-family: "Cabin Variable", sans-serif;
      font-size: min(7.40px, 148vw / 90, 1vh);
      height: 100%;
      justify-content: space-between;
      padding: 4em;
      position: relative;
      width: 100%;

      border-color: light-dark(#1c1a19, #5f5e61);
      border-radius: 2.5em;
      border-bottom-right-radius: 15em;
      border-style: solid;
      border-width: .5em;
      box-shadow: 0 10px 30px -10px black;

      gb-switch {
        left: 6em;
        position: absolute;
        top: 1.25em;
        z-index: 1;
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
              border: .15em solid black;
              border-radius: 50%;
              display: block;
              left: 1em;
              position: relative;
              width: 1.8em;

              &[power] {
                background-color: red;
                box-shadow: 0 0 2em red;
              }
            }
          }
        }

        .logo {
          color: #204786;
          text-align: start;

          .brand {
            font-family: "Pretendo", sans-serif;
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
