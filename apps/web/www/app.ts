import { GameBoy } from "rugby-wasm";

import type { Screen } from "./rugby/screen";
import type { Stereo } from "./rugby/stereo";

import { SAMPLE } from "./rugby/stereo";

/**
 * Clock frequency.
 *
 * @remarks
 * Frequency of the on-board quartz crystal used for timekeeping.
 */
export const FRQ = 4194304;

/**
 * Clock divider.
 *
 * @remarks
 * As an optimization, we use a clock divider and simulate a batch of `DIV`
 * cycles at a frequency of `FRQ / DIV`.
 */
const DIV = 32;

/**
 * Application state.
 */
export class Application {
  /**
   * Application configuration.
   */
  cfg: {
    /** Running status. */
    run: boolean;
    /** Emulated clock speed. */
    spd: number;
    /** Clock thread handle. */
    tid?: number;
  } = {
    run: false,
    spd: 1.0,
  };

  /**
   * Emulator instance.
   */
  emu = new GameBoy();

  /**
   * Graphical state.
   */
  gui = {
    /** Speaker model. */
    apu: undefined as Stereo | undefined,
    /** Display model. */
    lcd: undefined as Screen | undefined,
  };

  /**
   * Request application redraw.
   */
  draw: () => void = () => {};

  /**
   * Play or pause emulation.
   */
  play(enable = true) {
    this.cfg.run = enable;
    this.draw();
  }

  /**
   * Change emulation speed.
   */
  tick(speed = 0) {
    // Clear emulation thread
    if (this.cfg.tid) clearInterval(this.cfg.tid);
    // Update internal speed
    this.cfg.spd = speed;
    // Start emulation thread
    if (this.cfg.spd)
      this.cfg.tid = setInterval(this.cycle.bind(this), 1000 / DIV);
  }

  /**
   * Emulates a single cycle.
   */
  private cycle() {
    // No-op if paused
    if (!this.cfg.run) return;

    // Iterate pending cycles
    for (let tick = 0; tick < (this.cfg.spd * FRQ) / DIV; tick++) {
      // Emulate a single cycle
      this.emu.cycle();
      // Sample audio output
      if (tick % Math.floor((this.cfg.spd * FRQ) / SAMPLE) === 0) {
        // Fetch current sample
        const sample = this.emu.sample();
        // Play sample
        this.gui.apu?.play(sample);
      }
      // Redraw on vertical sync
      if (this.gui.lcd && this.emu.vsync()) {
        // Get next frame
        const frame = this.emu.frame();
        // Redraw display
        this.gui.lcd.redraw(frame);
      }
    }
  }
}
