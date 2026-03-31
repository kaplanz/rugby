import { GameBoy } from "rugby-wasm";

import type { Screen } from "./parts/screen";
import type { Stereo } from "./parts/stereo";

import { SAMPLE } from "./parts/stereo";

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
export class App {
  /**
   * Application context.
   */
  ctx = {
    /** Running status. */
    run: false,
    /** Emulated clock speed. */
    spd: 1.0,
    /** Clock thread handle. */
    tid: undefined as number | undefined,
    /** Audio sample accumulator. */
    acc: 0,
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
    this.ctx.run = enable;
    this.draw();
  }

  /**
   * Change emulation speed.
   */
  tick(speed = 0) {
    // Clear emulation thread
    if (this.ctx.tid) clearInterval(this.ctx.tid);
    // Update internal speed
    this.ctx.spd = speed;
    // Start emulation thread
    if (this.ctx.spd)
      this.ctx.tid = setInterval(this.cycle.bind(this), 1000 / DIV);
  }

  /**
   * Emulates a single cycle.
   */
  private cycle() {
    // No-op if paused
    if (!this.ctx.run) return;

    // Iterate pending cycles
    for (let tick = 0; tick < (this.ctx.spd * FRQ) / DIV; tick++) {
      // Emulate a single cycle
      this.emu.cycle();

      // Advance fractional accumulator
      this.ctx.acc += SAMPLE;
      // Sample audio output on each accumulated period
      if (this.ctx.acc >= this.ctx.spd * FRQ) {
        this.ctx.acc -= this.ctx.spd * FRQ;
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
