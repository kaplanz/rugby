import { LitElement, css, html, unsafeCSS } from "lit";
import { customElement, property, query } from "lit/decorators.js";

import type { Application } from "../app";

import type { SlDialog, SlTabGroup } from "@shoelace-style/shoelace";

import "@shoelace-style/shoelace/dist/components/dialog/dialog";
import "@shoelace-style/shoelace/dist/components/input/input";
import "@shoelace-style/shoelace/dist/components/range/range";
import "@shoelace-style/shoelace/dist/components/switch/switch";
import "@shoelace-style/shoelace/dist/components/tab-group/tab-group";
import "@shoelace-style/shoelace/dist/components/tab-panel/tab-panel";
import "@shoelace-style/shoelace/dist/components/tab/tab";

import shoelaceLight from "@shoelace-style/shoelace/dist/themes/light.css?raw";
import shoelaceDark from "@shoelace-style/shoelace/dist/themes/dark.css?raw";
import fontAwesome from "@fortawesome/fontawesome-free/css/all.min.css?raw";

@customElement("gb-dialog")
export class Dialog extends LitElement {
  /** Application state. */
  @property()
  app!: Application;

  /** Dialog element. */
  @query("sl-dialog")
  private pum!: SlDialog;

  /** Tab group element. */
  @query("sl-tab-group")
  private tab!: SlTabGroup;

  /**
   * Show the dialog.
   */
  show() {
    this.pum.show();
    // Redraw based on app state
    this.requestUpdate();
  }

  /** Hide the dialog. */
  hide() {
    this.pum.hide();
  }

  /**
   * Handle a key event.
   */
  private handle(event: KeyboardEvent) {
    switch (event.key) {
      // Toggle the dialog
      case "?":
        this.pum.open ? this.hide() : this.show();
        break;
      // Show the dialog
      case "F1":
        this.show();
        break;
      // Hide the dialog
      case "Escape":
        this.hide();
        break;
      // Show indexed tab
      case "1":
      case "2":
      case "3":
      case "4":
      case "5":
      case "6":
      case "7":
      case "8":
      case "9": {
        // Query all tabs
        const tabs = Array.from(this.tab.querySelectorAll("sl-tab-panel"));
        // Get the key index
        const idx = Number(event.key);
        // Check the tab's name
        const tab = tabs[idx - 1];
        if (!tab) break;
        // Show selected tab
        this.tab.show(tab.name);
        // Show help dialog
        this.show();
        break;
      }
    }
  }

  /**
   * Change app configuration.
   */
  private settings = {
    /**
     * Change configured state.
     */
    run: (event: CustomEvent) => {
      // Extract updated value
      const state = (event.target as HTMLInputElement).checked;
      console.log(`updated state: ${state ? "running" : "paused"}`);
      // Update emulator state
      this.app.play(state);
    },
    /**
     * Change configured speed.
     */
    spd: (event: CustomEvent) => {
      // Extract updated speed
      const speed = Number((event.target as HTMLInputElement).value);
      console.log(`updated speed: ${speed}`);
      // Update emulator speed
      this.app.tick(speed);
    },
  };

  connectedCallback() {
    super.connectedCallback();

    // Key bindings
    window.addEventListener("keydown", this.handle.bind(this));
  }

  disnnectedCallback() {
    super.disconnectedCallback();

    // Key bindings
    window.removeEventListener("keydown", this.handle.bind(this));
  }

  render() {
    return html`
      <button id="show" @click="${this.show}">&#x2139;&#xfe0e;</button>
      <sl-dialog>
        <sl-tab-group>

          <!-- About -->
          <sl-tab slot="nav" panel="about">About</sl-tab>
          <sl-tab-panel name="about">
            <article>
              <h3>Welcome</h3>
              <p>
                Rugby is an open-source Game Boy emulator, focused on
                cycle-accuracy and ease of use. It aims to support advanced
                emulation features including extensive debugging support and
                eventually tool-assisted speedrun capabilities.
              </p>
                This webpage hosts an online emulation experience, running
                in-browser via WebAssembly, meaning this page is statically
                served and emulation is happening locally on your device.
              <p>
              </p>
              <h3>Support</h3>
              <p>
                If you like this project, please consider supporting me by
                starring the repo on GitHub!
              </p>
            </article>
            <footer>
              <nav>
                <a href="https://github.com/kaplanz/rugby" target=”_blank”>
                  <i class="fa-brands fa-github"></i>
                </a>
                <a href="https://zakhary.dev" target=”_blank”>
                  <i class="fa-solid fa-link"></i>
                </a>
              <nav>
            </footer>
          </sl-tab-panel>

          <!-- Usage -->
          <sl-tab slot="nav" panel="usage">Usage</sl-tab>
          <sl-tab-panel name="usage">
            <article>
              <h3>Controls</h3>
              <p>
                Emulator controls should be fairly self-explanatory, with
                on-screen buttons corresponding to their emulator inputs. You
                toggle power by clicking the "OFF &bullet; ON" label on the
                frame. This menu can be opened by clicking the
                (&#x2139;&#xfe0e;) button.
              </p>
              <h3>Bindings</h3>
              <p>
                In addition to the on-screen buttons, these key bindings can be
                used as joypad inputs:
              </p>
              <table>
                <thead>
                  <tr><th>Keyboard</th><th>Emulator</th></tr>
                </thead>
                <tbody>
                  <tr><td>X</td><td>A</td></tr>
                  <tr><td>Z</td><td>B</td></tr>
                  <tr><td>Enter</td><td>Start</td></tr>
                  <tr><td>Space</td><td>Select</td></tr>
                  <tr><td>Arrows</td><td>D-pad</td></tr>
                </tbody>
              </table>
              <p>
                There are also key bindings for the application:
              </p>
              <table>
                <thead>
                  <tr><th>Keyboard</th><th>Action</th></tr>
                </thead>
                <tbody>
                  <tr><td>?</td><td>Toggle menu</td></tr>
                  <tr><td>F1</td><td>Show menu</td></tr>
                  <tr><td>Escape</td><td>Hide menu</td></tr>
                  <tr><td>1-9</td><td>Open menu tab</td></tr>
                </tbody>
              </table>
            </article>
          </sl-tab-panel>

          <!-- Settings -->
          <sl-tab slot="nav" panel="settings">Settings</sl-tab>
          <sl-tab-panel name="settings">
            <!-- Enable -->
            <sl-switch
              ?checked=${this.app.cfg.run}
              @sl-change=${this.settings.run.bind(this)}
            >
              <span>Enable</span>
              <span slot="help-text">
                When disabled, the emulator will be paused.
              </span>
            </sl-switch>
            <!-- Speed -->
            <sl-range
              min="0"
              max="3"
              step=".1"
              value=${this.app.cfg.spd}
              @sl-change=${this.settings.spd.bind(this)}
            >
              <span slot="label">Speed</span>
              <span slot="help-text">
                Controls the simulated clock speed of the emulator.
              </span>
            </sl-range>
          </sl-tab-panel>

        </sl-tab-group>
      </sl-dialog>
    `;
  }

  static styles = [
    css`
      :host {
        font-family: "Cabin Variable";
      }

      @media (prefers-color-scheme: light) {
        ${unsafeCSS(shoelaceLight)}
      }

      @media (prefers-color-scheme: dark) {
        ${unsafeCSS(shoelaceDark)}
      }

      #show {
        position: absolute;
        right: 0;
        z-index: 1;

        box-sizing: border-box;
        height: 1.5em;
        width: 1.5em;
        margin: 1em;
        padding: 0;

        border-color: light-dark(#1c1a19, #5f5e61);
        border-radius: 20px;
        border-style: solid;
        border-width: 2px;
        box-shadow: 0 5px 15px -5px black;

        background-color: light-dark(#c5c0bd, #1c1a19);
        color: inherit;
        cursor: pointer;
        font-size: 1.3em;

        &:active {
          filter: brightness(60%);
        }
      }

      sl-dialog {
        &::part(header) {
          display: none;
        }
      }

      sl-tab-group {
        height: 60vh;

        &::part(base),
        &::part(body) {
          height: 100%;
        }
      }

      sl-tab-panel {
        height: 100%;
        position: relative;

        &::part(base) {
          height: 100%;
        }

        &[name="about"] {
          &::part(base) {
            display: flex;
            flex-direction: column;
          }

          > article {
            flex: 1 1 auto;
          }

          > footer {
            border-top: 1px solid light-dark(#e4e4e7, #36363b);
            margin-top: .5em;
            padding-top: 1em;

            nav {
              display: flex;
              font-size: 1.3em;
              justify-content: center;
              gap: .5em;

              a {
                color: inherit;
                text-decoration: none;
              }
            }
          }
        }

        &[name="settings"]::part(base) {
          display: flex;
          flex-direction: column;
          gap: 1em;
        }
      }

      sl-switch {
        &::part(label) {
          order: -1;
          margin: 0 8px 0 0;
        }
      }

      sl-range {
        &::part(form-control) {
          display: flex;
          flex-wrap: wrap;
          align-items: center;
        }

        &::part(form-control-label) {
          flex: 0 1 auto;
          margin: 0;
        }

        &::part(form-control-input) {
          flex: 1 1 0;
          margin: 0 .5em;
        }

        &::part(form-control-help-text) {
          flex: 1 1 100%;
        }
      }

      article {
        h1,
        h2,
        h3,
        h4,
        h5,
        h6 {
          margin-top: 0;
        }

        table {
          width: 100%;
          border-collapse: collapse;
          margin: 20px 0;

          th, td {
            border: 1px solid light-dark(#ddd, #444);
            padding: 8px;
            text-align: center;
          }

          th {
            border-bottom-width: 3px;
            font-weight: bold;
          }
        }
      }
    `,
    unsafeCSS(fontAwesome),
  ];
}
