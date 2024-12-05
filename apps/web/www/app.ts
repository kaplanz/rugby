import { GameBoy } from "rugby-web";

/**
 * Application state.
 */
export class Application {
    /**
     * Application configuration.
     */
    cfg = {
        /** Running status. */
        run: false,
        /** Emulated clock speed. */
        spd: 1.0,
    };

    /**
     * Emulator instance.
     */
    emu = new GameBoy();

    /**
     * Update requested callback.
     */
    callback: () => void = () => {};

    /**
     * Enables emulation.
     */
    play(enable: boolean) {
        this.cfg.run = enable;
        this.callback();
    }
}
