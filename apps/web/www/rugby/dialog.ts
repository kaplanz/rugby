import { LitElement, css, html, unsafeCSS } from "lit";
import { customElement, property, query, state } from "lit/decorators.js";

import { Cartridge } from "rugby-web";

import type { Application } from "../app";

import type { SlDialog, SlTabGroup } from "@shoelace-style/shoelace";

import "@shoelace-style/shoelace/dist/components/badge/badge";
import "@shoelace-style/shoelace/dist/components/button/button";
import "@shoelace-style/shoelace/dist/components/dialog/dialog";
import "@shoelace-style/shoelace/dist/components/divider/divider";
import "@shoelace-style/shoelace/dist/components/input/input";
import "@shoelace-style/shoelace/dist/components/range/range";
import "@shoelace-style/shoelace/dist/components/switch/switch";
import "@shoelace-style/shoelace/dist/components/tab-group/tab-group";
import "@shoelace-style/shoelace/dist/components/tab-panel/tab-panel";
import "@shoelace-style/shoelace/dist/components/tab/tab";

import fontAwesome from "@fortawesome/fontawesome-free/css/all.min.css?raw";
import shoelaceDark from "@shoelace-style/shoelace/dist/themes/dark.css?raw";
import shoelaceLight from "@shoelace-style/shoelace/dist/themes/light.css?raw";

/**
 * Library database name.
 */
const LIB = "games";

/**
 * Library game.
 */
interface Game {
  /** Identifier */
  id: number;
  /** File name */
  name: string;
  /** ROM data */
  data: Uint8Array;
}

@customElement("gb-dialog")
export class Dialog extends LitElement {
  /** Application state. */
  @property()
  app!: Application;

  /** Game library. */
  private lib: Array<Game> = [];

  /** Running status. */
  @state()
  private run?: boolean;

  /** Dialog element. */
  @query("sl-dialog")
  private pum!: SlDialog;

  /** Tab group element. */
  @query("sl-tab-group")
  private tab!: SlTabGroup;

  /** File select input. */
  @query("input[type=file]")
  private rom!: HTMLInputElement;

  /**
   * Show the dialog.
   */
  show() {
    this.pum.show();
  }

  /**
   * Show dialog event trigger.
   */
  private onShow() {
    // Store emulator state
    this.run = this.app.cfg.run;
    // Pause the emulator
    this.app.play(false);
  }

  /** Hide the dialog. */
  hide() {
    this.pum.hide();
  }

  /**
   * Hide dialog event trigger.
   */
  private onHide() {
    // Restore emulator state
    this.app.play(this.run);
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
   * Manage game library database.
   */
  private database = {
    /**
     * Init the library database.
     */
    init: async () => {
      return new Promise<IDBDatabase>((resolve, reject) => {
        const rq = indexedDB.open(LIB, 1);
        rq.onupgradeneeded = (_) => {
          const db = rq.result;
          if (!db.objectStoreNames.contains(LIB)) {
            db.createObjectStore(LIB, { keyPath: "id", autoIncrement: true });
          }
        };
        rq.onsuccess = () => resolve(rq.result);
        rq.onerror = () => reject(rq.error);
      });
    },

    /**
     * Load the library database.
     */
    load: async () => {
      const db = await this.database.init();
      const tx = db.transaction(LIB, "readonly");
      const st = tx.objectStore(LIB);
      const rq = st.getAll();
      rq.onsuccess = () => {
        this.lib = rq.result;
        console.log(`loaded library: ${this.lib.map((game) => game.name)}`);
        this.requestUpdate();
      };
    },

    /**
     * Drop the library database.
     */
    drop: async () => {
      return new Promise<void>((resolve, reject) => {
        const rq = indexedDB.deleteDatabase(LIB);
        rq.onsuccess = () => {
          this.lib = [];
          console.log("dropped library");
          this.requestUpdate();
          resolve();
        };
        rq.onerror = () => reject(rq.error);
      });
    },

    /**
     * Add a game to the library.
     */
    add: async (game: { name: string; data: Uint8Array }) => {
      const db = await this.database.init();
      const tx = db.transaction(LIB, "readwrite");
      const st = tx.objectStore(LIB);
      st.add(game);
      tx.oncomplete = () => this.database.load();
    },

    /**
     * Queries a game in the library.
     */
    query: async (id: number) => {
      const db = await this.database.init();
      const tx = db.transaction(LIB, "readonly");
      const st = tx.objectStore(LIB);
      const rq = st.get(id);
      return new Promise<Game>((resolve, reject) => {
        rq.onsuccess = () => resolve(rq.result);
        rq.onerror = () => reject(rq.result);
      });
    },

    /**
     * Delete a game from the library.
     */
    delete: async (id: number) => {
      const db = await this.database.init();
      const tx = db.transaction(LIB, "readwrite");
      const st = tx.objectStore(LIB);
      st.delete(id);
      tx.oncomplete = () => this.database.load();
    },

    /**
     * Uploads a game from a file.
     * */
    upload: async (file: File) => {
      // Read the uploaded file
      const reader = new FileReader();
      reader.readAsArrayBuffer(file);

      // When ready, insert into the console
      reader.onload = (event: ProgressEvent<FileReader>) => {
        // Truncate file extension
        const name = file.name.split(".").slice(0, -1).join(".");

        // Load provided data
        let data: Uint8Array;
        try {
          // Attempt to fetch result
          const result = event.target?.result;
          if (!result) throw new Error("missing file data");
          // Convert to a byte array
          data = new Uint8Array(result as ArrayBuffer);
        } catch (error) {
          console.error(error);
          // Extract the error message
          const msg = error instanceof Error ? error.message : String(error);
          // Notify the user
          alert(msg);
          return;
        }

        // Add to the database
        this.database.add({ name, data });
      };
    },

    /**
     * Plays a game form the library.
     * */
    play: async (id: number) => {
      // Pause emulator
      this.app.play(false);
      // Retrieve game data
      const game = await this.database.query(id);
      // Construct a cartridge
      const cart = new Cartridge(game.data);
      console.log("inserted game cartridge");
      // Reload emulator with cartridge
      this.app.emu.insert(cart);
      this.app.emu.reset();
      // Resume emulation
      this.app.play();
      // Hide the dialog
      this.hide();
    },
  };

  /**
   * Change app configuration.
   */
  private settings = {
    /**
     * Change configured state.
     */
    run: (event: CustomEvent) => {
      // Extract updated value
      this.run = (event.target as HTMLInputElement).checked;
      console.log(`updated state: ${this.run ? "running" : "paused"}`);
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
    // Game library
    this.database.load();
  }

  disnnectedCallback() {
    super.disconnectedCallback();

    // Key bindings
    window.removeEventListener("keydown", this.handle.bind(this));
  }

  render() {
    return html`
      <button id="show" @click="${this.show}">&#x2139;&#xfe0e;</button>
      <sl-dialog
        @sl-show=${this.onShow.bind(this)}
        @sl-hide=${this.onHide.bind(this)}
      >
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
                <a href="https://github.com/kaplanz/rugby/tree/${
                  // @ts-ignore
                  BUILD.COMMIT
                }" target=”_blank”>
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
                can toggle power by clicking the "OFF &bullet; ON" label on the
                frame. Open the menu by clicking the (&#x2139;&#xfe0e;) button.
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
                  <tr><td>Backspace</td><td>Select</td></tr>
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

          <!-- Library -->
          <sl-tab slot="nav" panel="library">Library</sl-tab>
          <sl-tab-panel name="library">
            <div class="menu">
              <sl-button @click=${() => this.rom.click()}>
                <span>Upload</span>
                <i slot="suffix" class="fa-solid fa-file-arrow-up"></i>
                <input
                  type="file"
                  accept=".gb,.gbc"
                  style="display: none;"
                  @change=${
                    // biome-ignore lint/style/noNonNullAssertion: none
                    () => this.database.upload(this.rom.files?.item(0)!)
                  }
                />
              </sl-button>
              <sl-button
                variant="danger"
                outline
                @click=${this.database.drop.bind(this)}
              >
                <span>Clear</span>
                <i slot="suffix" class="fa-regular fa-trash-can"></i>
              </sl-button>
            </div>
            <sl-divider></sl-divider>
            <ul class="games">
              ${this.lib.map(
                (game) => html`
                <li>
                  <sl-button
                    variant="primary"
                    outline
                    @click=${() => this.database.play(game.id)}
                  >
                    <i class="fa-solid fa-play"></i>
                  </sl-button>
                  <span>${game.name}</span>
                  <sl-button
                    variant="danger"
                    outline
                    @click=${() => this.database.delete(game.id)}
                  >
                    <i class="fa-solid fa-xmark"></i>
                  </sl-button>
                </li>
              `,
              )}
            </ul>
          </sl-tab-panel>

          <!-- Settings -->
          <sl-tab slot="nav" panel="settings">Settings</sl-tab>
          <sl-tab-panel name="settings">
            <!-- Enable -->
            <sl-switch
              ?checked=${this.run}
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

        &[name="library"] {
          sl-button[variant="danger"] {
            margin-left: auto;
          }

          div.menu {
            display: flex;
            gap: .5em;
          }

          ul.games {
            display: flex;
            flex-direction: column;
            gap: .5em;

            list-style-type: none;
            padding: 0;

            li {
              display: flex;
              align-items: center;
              gap: .5em;

              sl-button::part(base) {
                border-width: 0;
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
