import { LitElement, css, html } from "lit";
import { customElement, query, state } from "lit/decorators.js";

import { Cartridge, demo } from "rugby-web";

import { Application } from "../app";
import type { Dialog } from "./dialog";
import type { Screen } from "./screen";

import "./dialog";
import "./joypad";
import "./screen";
import "./stereo";
import "./switch";

/**
 * Game Boy emulator.
 */
@customElement("game-boy")
export class GameBoy extends LitElement {
  /** Application state. */
  @state()
  private app = new Application();

  /** Graphical display model. */
  @query("gb-screen")
  private lcd!: Screen;

  /** Graphical display model. */
  @query("gb-dialog")
  private cfg!: Dialog;

  /**
   * Opens menu dialog.
   */
  menu() {
    this.cfg.show();
  }

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
        touch-action: none;
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

      background-color: light-dark(#c5c0bd, #1c1a19);
      color: #204786;
      font-family: "Cabin Variable", sans-serif;
      font-size: min(7.40px, 148dvw / 90, 1dvh);

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
