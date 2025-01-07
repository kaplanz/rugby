//! Function callbacks.

use std::sync::OnceLock;

use super::{unsigned, void};
// documentation uses
#[allow(unused_imports)]
use crate::*;

/// Environment callback to give implementations a way of performing
/// uncommon tasks.
///
/// # Note
///
/// Extensible.
///
/// # Parameters
///
/// - `cmd`: The command to run.
/// - `data`: A pointer to the data associated with the command.
///
/// # Returns
///
/// Varies by callback, but will always return `false` if the command is not
/// recognized.
///
/// # See
///
/// - [`RETRO_ENVIRONMENT_SET_ROTATION`]
/// - [`retro_set_environment`]
pub type retro_environment_t = extern "C" fn(cmd: unsigned, data: *const void) -> bool;

/// Environment callback.
pub(crate) static ENVIRONMENT: OnceLock<retro_environment_t> = OnceLock::new();

/// Render a frame.
///
/// # Note
///
/// For performance reasons, it is highly recommended to have a frame that
/// is packed in memory, i.e. `pitch == width * byte_per_pixel`. Certain
/// graphic APIs, such as OpenGL ES, do not like textures that are not
/// packed in memory.
///
/// # Parameters
///
/// - `data`: A pointer to the frame buffer data with a pixel format of
///   15-bit `0RGB1555` native endian, unless changed with
///   [`RETRO_ENVIRONMENT_SET_PIXEL_FORMAT`].
/// - `width`: The width of the frame buffer, in pixels.
/// - `height`: The height frame buffer, in pixels.
/// - `pitch`: The width of the frame buffer, in bytes.
///
/// # See
///
/// - [`retro_set_video_refresh`]
/// - [`RETRO_ENVIRONMENT_SET_PIXEL_FORMAT`]
/// - [`retro_pixel_format`]
pub type retro_video_refresh_t =
    extern "C" fn(data: *const void, width: unsigned, height: unsigned, pitch: usize);

/// Video refresh callback.
pub(crate) static VIDEO_REFRESH: OnceLock<retro_video_refresh_t> = OnceLock::new();

/// Renders a single audio frame.
///
/// Should only be used if implementation generates a single sample at a
/// time.
///
/// # Parameters
///
/// - `left`: The left audio sample represented as a signed 16-bit native
///   endian.
/// - `right`: The right audio sample represented as a signed 16-bit native
///   endian.
///
/// # See
///
/// - [`retro_set_audio_sample`]
/// - [`retro_set_audio_sample_batch`]
pub type retro_audio_sample_t = extern "C" fn(left: i16, right: i16);

/// Audio sample callback.
pub(crate) static AUDIO_SAMPLE: OnceLock<retro_audio_sample_t> = OnceLock::new();

/// Renders multiple audio frames in one go.
///
/// # Note
///
/// Only one of the audio callbacks must ever be used.
///
/// - `data`: A pointer to the audio sample data pairs to render.
/// - `frames`: The number of frames that are represented in the data. One
///    frame is defined as a sample of left and right channels, interleaved.
///    For example: `int16_t buf[4] = { l, r, l, r };` would be 2 frames.
///
/// # Returns
///
/// The number of frames that were processed.
///
/// # See
///
/// - [`retro_set_audio_sample_batch`]
/// - [`retro_set_audio_sample`]
pub type retro_audio_sample_batch_t = extern "C" fn(data: *const i16, frames: usize) -> usize;

/// Audio sample batch callback.
pub(crate) static AUDIO_SAMPLE_BATCH: OnceLock<retro_audio_sample_batch_t> = OnceLock::new();

/// Polls input.
///
/// # See
///
/// - [`retro_set_input_poll`]
pub type retro_input_poll_t = extern "C" fn();

/// Input poll callback.
pub(crate) static INPUT_POLL: OnceLock<retro_input_poll_t> = OnceLock::new();

/// Queries for input for player 'port'.
///
/// - `port`: Which player 'port' to query.
/// - `device`: Which device to query for. Will be masked with
///   [`RETRO_DEVICE_MASK`].
/// - `index`: The input index to retrieve. The exact semantics depend on
///   the device type given in `device`.
/// - `id`: The ID of which value to query, like
///   [`RETRO_DEVICE_ID_JOYPAD_B`].
///
///
/// # Returns
///
/// Depends on the provided arguments, but will return 0 if their values are
/// unsupported by the frontend or the backing physical device.
///
/// # Note
///
/// Specialization of devices such as `RETRO_DEVICE_JOYPAD_MULTITAP` that have
/// been set with [`retro_set_controller_port_device`] will still use the higher
/// level [`RETRO_DEVICE_JOYPAD`] to request input.
///
/// - [`retro_set_input_state`]
/// - [`RETRO_DEVICE_NONE`]
/// - [`RETRO_DEVICE_JOYPAD`]
/// - [`RETRO_DEVICE_MOUSE`]
/// - [`RETRO_DEVICE_KEYBOARD`]
/// - [`RETRO_DEVICE_LIGHTGUN`]
/// - [`RETRO_DEVICE_ANALOG`]
/// - [`RETRO_DEVICE_POINTER`]
pub type retro_input_state_t =
    extern "C" fn(port: unsigned, device: unsigned, index: unsigned, id: unsigned) -> i16;

/// Input state callback.
pub(crate) static INPUT_STATE: OnceLock<retro_input_state_t> = OnceLock::new();
