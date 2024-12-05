import { LitElement, css, html } from "lit";
import { customElement, property } from "lit/decorators.js";

import { Cartridge } from "rugby-web";

import { Application } from "../app";

@customElement("gb-loader")
export class Loader extends LitElement {
  /** Application state. */
  @property()
  app!: Application;

  private upload(files: File[]) {
    // Ensure something has been uploaded
    if (!files[0]) return;

    // Read the uploaded file
    const reader = new FileReader();
    reader.readAsArrayBuffer(files[0]);

    // When ready, insert into the console
    reader.onload = (event: ProgressEvent<FileReader>) => {
      // Pause emulator
      this.app.play(false);

      // Load provided data
      try {
        // Attempt to fetch result
        const result = event.target?.result;
        if (!result) throw new Error("missing file data");
        // Convert to an array
        const data = new Uint8Array(result as ArrayBuffer);
        // Construct a cartridge
        const cart = new Cartridge(data);
        console.log("inserted game cartridge");
        // Insert into emulator
        this.app.emu.insert(cart);
        // Perform hardware reset
        this.app.emu.reset();
        // Resume emulation
        this.app.play(true);
      } catch (error) {
        console.error(error);
        // Extract the error message
        const msg = error instanceof Error ? error.message : String(error);
        // Notify the user
        alert(msg);
      }

      // Update reactive state
      this.requestUpdate();
    };
  }

  private handle(event: Event) {
    const input = event.target as HTMLInputElement;
    const files = input.files ? Array.from(input.files) : [];
    this.upload(files);
  }

  render() {
    return html`
      <label>
        <input
          type="file"
          accept=".gb,.gbc"
          @change=${this.handle.bind(this)}
        />
      </label>
    `;
  }

  static styles = css`
    :host {
      display: block;
    }
  `;
}
