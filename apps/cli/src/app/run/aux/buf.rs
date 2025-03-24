//! Audio sample buffers.

use ringbuf::HeapRb as Ring;
use ringbuf::traits::{Consumer, Observer, Producer};
use rubato::{
    Resampler,
    SincFixedIn,
    SincInterpolationParameters,
    SincInterpolationType,
    WindowFunction,
};
use rugby::emu::part::audio::Sample;

/// Audio latency maximum (in milliseconds).
const DELAY: usize = super::LATENCY;

/// Audio channel count.
const NCHAN: usize = super::CHANNELS;

/// Audio resampling stream.
pub struct Stream {
    /// Sample rate input.
    #[expect(unused)]
    ifrq: u32,
    /// Sample rate output.
    ofrq: u32,
    /// Sample buffer input.
    ibuf: Ring<Sample>,
    /// Sample buffer output.
    obuf: Ring<Sample>,
    /// Sample rate converter.
    sinc: SincFixedIn<f32>,
    /// Working buffer input.
    iwrk: Vec<Vec<f32>>,
    /// Working buffer output.
    owrk: Vec<Vec<f32>>,
    /// Frames until resample.
    ///
    /// Number of input frames needed for next resampling operation.
    need: usize,
}

impl Stream {
    /// Constructs a new `Stream`.
    pub fn new(ifrq: u32, ofrq: u32) -> Self {
        // Calculate buffer sizes based on rates and duration
        let ilen = ifrq as usize * DELAY / 1000;
        let olen = ofrq as usize * DELAY / 1000;

        // Create interface buffers
        let ibuf = Ring::new(ilen);
        let obuf = Ring::new(olen);

        // Configure resampler parameters optimized for Game Boy audio
        let params = SincInterpolationParameters {
            sinc_len: 128,
            f_cutoff: 0.95,
            interpolation: SincInterpolationType::Linear,
            oversampling_factor: 128,
            window: WindowFunction::BlackmanHarris2,
        };

        // Create the resampler with appropriate parameters
        let sinc = SincFixedIn::<f32>::new(
            f64::from(ofrq) / f64::from(ifrq),
            1.0,
            params,
            ilen / 4,
            NCHAN,
        )
        .unwrap();

        // Get initial frame counts
        let need = sinc.input_frames_next();
        let omax = sinc.output_frames_max();

        // Create working buffers for the resampler
        let iwrk = vec![vec![0.0; need]; NCHAN];
        let owrk = vec![vec![0.0; omax]; NCHAN];

        Self {
            ifrq,
            ofrq,
            ibuf,
            obuf,
            sinc,
            iwrk,
            owrk,
            need,
        }
    }

    /// Pushes a sample to the input buffer.
    ///
    /// # Note
    ///
    /// Processes if enough samples are available.
    pub fn push(&mut self, sample: Sample) -> bool {
        // Add sample to input buffer
        let res = self.ibuf.try_push(sample).is_ok();

        // Process when we have enough samples
        if self.ibuf.occupied_len() >= self.need {
            self.process();
        }

        res
    }

    /// Pulls a sample from the output buffer, potentially triggering processing if buffer is low.
    ///
    /// # Returns
    ///
    /// An audio sample if available, or `None` if the buffer is empty.
    pub fn pull(&mut self) -> Option<Sample> {
        // Process more samples if output buffer is getting low
        if self.obuf.occupied_len() < self.ofrq as usize / 100
            && self.ibuf.occupied_len() >= self.need
        {
            self.process();
        }

        self.obuf.try_pop()
    }

    /// Processes samples from input buffer to output buffer using the resampler.
    fn process(&mut self) {
        // Fill resampler input buffer from our input buffer
        let mut count = 0;
        while count < self.need && !self.ibuf.is_empty() {
            if let Some(sample) = self.ibuf.try_pop() {
                self.iwrk[0][count] = sample.lt;
                self.iwrk[1][count] = sample.rt;
                count += 1;
            } else {
                break;
            }
        }

        // Only process if we filled the buffer completely
        if count == self.need {
            // Perform resampling
            let (_, count) = self
                .sinc
                .process_into_buffer(&self.iwrk, &mut self.owrk, None)
                .unwrap();

            // Transfer resampled data to output buffer
            for idx in 0..count {
                let _ = self.obuf.try_push(Sample {
                    lt: self.owrk[0][idx],
                    rt: self.owrk[1][idx],
                });
            }

            // Update number of frames needed for next processing
            self.need = self.sinc.input_frames_next();
        } else {
            // Return samples to input buffer if we couldn't get enough
            for idx in (0..count).rev() {
                let _ = self.ibuf.try_push(Sample {
                    lt: self.iwrk[0][idx],
                    rt: self.iwrk[1][idx],
                });
            }
        }
    }
}
