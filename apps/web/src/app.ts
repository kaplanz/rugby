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
    /** Animation frame handle. */
    tid: undefined as number | undefined,
    /** Audio sample accumulator. */
    acc: 0,
    /** Previous frame timestamp. */
    prv: undefined as number | undefined,
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
    // Stop loop
    if (this.ctx.tid) {
      cancelAnimationFrame(this.ctx.tid);
      this.ctx.tid = undefined;
    }
    // Update speed
    this.ctx.spd = speed;
    // Start loop
    if (this.ctx.spd)
      this.ctx.tid = requestAnimationFrame(this.loop.bind(this));
  }

  /**
   * Runs one animation frame.
   */
  private loop(now: number) {
    // Schedule next frame
    this.ctx.tid = requestAnimationFrame(this.loop.bind(this));

    // No-op if not running
    if (!this.ctx.run) return;

    // Compute elapsed time
    //
    // Number of cycles to emulate depends on much time has elapsed since the
    // last frame. Avoid runaway emulation by limiting to at most 100ms.
    const delta = Math.min(now - (this.ctx.prv ?? now), 100) / 1000;
    // Update timestamp
    this.ctx.prv = now;

    // Iterate pending cycles
    for (let tick = 0; tick < this.ctx.spd * FRQ * delta; tick++) {
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
