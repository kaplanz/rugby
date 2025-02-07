/// <reference lib="webworker"/>

class ChiptuneWorklet extends AudioWorkletProcessor {
  constructor() {
    super();

    // Listen for incoming messages from the main thread.
    this.port.onmessage = (event) => {
      this.buf.push(event.data);
    };

    // Use a buffer to store samples.
    this.buf = [];
    // Retain last valid sample (avoids audio pops).
    this.out = undefined;
  }

  process(_, outputs) {
    const output = outputs[0];
    const channel = {
      lt: output[0],
      rt: output[1],
    };

    for (let i = 0; i < channel.lt.length; i++) {
      // Dequeue the next sample
      const sample = this.buf.shift();
      // Use channel values
      channel.lt[i] = sample?.lt ?? this.out?.lt ?? 0;
      channel.rt[i] = sample?.rt ?? this.out?.rt ?? 0;
      // Update last valid sample
      this.out = sample ?? this.out;
    }
    // Slowly decay stale sample
    //
    // This will prevent pops as the volume is adjusted while paused, as samples
    // which would otherwise be reused could have a DC offset. By tending them
    // towards zero, that offset is no longer multiplied as volume is adjusted.
    if (this.out) {
      this.out.lt *= 0.9;
      this.out.rt *= 0.9;
    }

    return true;
  }
}

registerProcessor("chiptune", ChiptuneWorklet);
