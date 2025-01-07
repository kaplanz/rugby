//! Input devices.
//!
//! `libretro`'s fundamental device abstractions.
//!
//! `libretro`'s input system consists of abstractions over standard device
//! types, such as a joypad (with or without analog), mouse, keyboard, light
//! gun, or an abstract pointer. Instead of managing input devices themselves,
//! cores need only to map their own concept of a controller to libretro's
//! abstractions. This makes it possible for frontends to map the abstract types
//! to a real input device without having to worry about the correct use of
//! arbitrary (real) controller layouts.

use super::unsigned;
// documentation uses
#[allow(unused_imports)]
use crate::*;

pub const RETRO_DEVICE_TYPE_SHIFT: unsigned = 8;

#[allow(non_snake_case)]
pub const RETRO_DEVICE_MASK: unsigned = (1 << RETRO_DEVICE_TYPE_SHIFT) - 1;

/// Defines an ID for a subclass of a known device type.
///
/// To define a subclass ID, use this macro like so:
///
/// ```c
/// const RETRO_DEVICE_SUPER_SCOPE RETRO_DEVICE_SUBCLASS(RETRO_DEVICE_LIGHTGUN, 1)
/// const RETRO_DEVICE_JUSTIFIER RETRO_DEVICE_SUBCLASS(RETRO_DEVICE_LIGHTGUN, 2)
/// ```
///
/// Correct use of this macro allows a frontend to select a suitable physical
/// device to map to the emulated device.
///
/// # Note
///
/// Cores must use the base ID when polling for input, and frontends must only
/// accept the base ID for this purpose. Polling for input using subclass IDs is
/// reserved for future definition.
///
/// # Parameters
///
/// - `base`: One of the `RETRO_DEVICE` "base device types".
/// - `id`: A unique ID, with respect to `base`. Must be a non-negative integer.
///
/// # Returns
///
/// A unique subclass ID.
///
/// # See
///
/// - [`retro_controller_description`]
/// - [`retro_set_controller_port_device`]
#[allow(non_snake_case)]
#[must_use]
pub const fn RETRO_DEVICE_SUBCLASS(base: unsigned, id: unsigned) -> unsigned {
    ((id + 1) << RETRO_DEVICE_TYPE_SHIFT) | base
}

/* Input Device Classes */

/// Indicates no input.
///
/// When provided as the `device` argument to [`retro_input_state_t`], all other
/// arguments are ignored and zero is returned.
///
/// # See
///
/// - [`retro_input_state_t`]
pub const RETRO_DEVICE_NONE: unsigned = 0;

/// An abstraction around a game controller, known as a "RetroPad".
///
/// The RetroPad is modelled after a SNES controller, but with additional
/// L2/R2/L3/R3 buttons (similar to a PlayStation controller).
///
/// When provided as the `device` argument to [`retro_input_state_t`], the `id`
/// argument denotes the button (including D-Pad directions) to query. The
/// result of said query will be 1 if the button is down, 0 if not.
///
/// There is one exception; if [`RETRO_DEVICE_ID_JOYPAD_MASK`] is queried (and
/// the frontend supports this query), the result will be a bitmask of all
/// pressed buttons.
///
/// # See
///
/// - [`retro_input_state_t`]
/// - [`RETRO_DEVICE_ANALOG`]
/// - `RETRO_DEVICE_ID_JOYPAD`
/// - [`RETRO_DEVICE_ID_JOYPAD_MASK`]
/// - [`RETRO_ENVIRONMENT_GET_INPUT_BITMASKS`]
pub const RETRO_DEVICE_JOYPAD: unsigned = 1;

/// An abstraction around a mouse, similar to the SNES Mouse but with more
/// buttons.
///
/// When provided as the `device` argument to `retro_input_state_t`, the `id`
/// argument denotes the button or axis to query. For buttons, the result of
/// said query will be 1 if the button is down or 0 if not. For mouse wheel
/// axes, the result will be 1 if the wheel was rotated in that direction and 0
/// if not. For the mouse pointer axis, the result will be thee mouse's movement
/// relative to the last poll. The core is responsible for tracking the mouse's
/// position, and the frontend is responsible for preventing interference by the
/// real hardware pointer (if applicable).
///
/// # Note
///
/// This should only be used for cores that emulate mouse input, such as for
/// home computers or consoles with mouse attachments. Cores that emulate light
/// guns should use [`RETRO_DEVICE_LIGHTGUN`], and cores that emulate touch
/// screens should use [`RETRO_DEVICE_POINTER`].
///
/// # See
///
/// - [`RETRO_DEVICE_POINTER`]
/// - [`RETRO_DEVICE_LIGHTGUN`]
pub const RETRO_DEVICE_MOUSE: unsigned = 2;

/// An abstraction around a keyboard.
///
/// When provided as the `device` argument to [`retro_input_state_t`], the `id`
/// argument denotes the key to poll.
///
/// # Note
///
/// This should only be used for cores that emulate keyboard input, such as for
/// home computers or consoles with keyboard attachments. Cores that emulate
/// gamepads should use [`RETRO_DEVICE_JOYPAD`] or [`RETRO_DEVICE_ANALOG`], and
/// leave keyboard compatibility to the frontend.
///
/// # See
///
/// - [`RETRO_ENVIRONMENT_SET_KEYBOARD_CALLBACK`]
/// - [`retro_key`]
pub const RETRO_DEVICE_KEYBOARD: unsigned = 3;

/// An abstraction around a light gun, similar to the PlayStation's Guncon.
///
/// When provided as the `device` argument to [`retro_input_state_t`], the `id`
/// argument denotes one of several possible inputs.
///
/// The gun's coordinates are reported in screen space (similar to the pointer)
/// in the range of `[-0x8000, 0x7fff]`. Zero is the center of the game's screen
/// and `-0x8000` represents out-of-bounds. The trigger and various auxiliary
/// buttons are also reported.
///
/// # Note
///
/// A forced off-screen shot can be requested for auto-reloading function in
/// some games.
///
/// # See
///
/// - [`RETRO_DEVICE_POINTER`]
pub const RETRO_DEVICE_LIGHTGUN: unsigned = 4;

/// An extension of the RetroPad that supports analog input.
///
/// The analog RetroPad provides two virtual analog sticks (similar to DualShock
/// controllers) and allows any button to be treated as analog (similar to Xbox
/// shoulder triggers).
///
/// When provided as the `device` argument to [`retro_input_state_t`], the `id`
/// argument denotes an analog axis or an analog button.
///
/// Analog axes are reported in the range of `[-0x8000, 0x7fff]`, with the X
/// axis being positive towards the right and the Y axis being positive towards
/// the bottom.
///
/// Analog buttons are reported in the range of `[0, 0x7fff]`, where 0 is
/// unpressed and 0x7fff is fully pressed.
///
/// # Note
///
/// Cores should only use this type if they need analog input. Otherwise,
/// [`RETRO_DEVICE_JOYPAD`] should be used.
///
/// # See
///
/// - [`RETRO_DEVICE_JOYPAD`]
pub const RETRO_DEVICE_ANALOG: unsigned = 5;

/// Input Device: Pointer.
///
/// Abstracts the concept of a pointing mechanism, e.g. touch. This allows
/// libretro to query in absolute coordinates where on the screen a mouse (or
/// something similar) is being placed. For a touch centric device, coordinates
/// reported are the coordinates of the press.
///
/// Coordinates in X and Y are reported as: `[-0x7fff, 0x7fff]`: `-0x7fff`
/// corresponds to the far left/top of the screen, and `0x7fff` corresponds to
/// the far right/bottom of the screen. The "screen" is here defined as area
/// that is passed to the frontend and later displayed on the monitor. If the
/// pointer is outside this screen, such as in the black surrounding areas when
/// actual display is larger, edge position is reported. An explicit edge
/// detection is also provided, that will return 1 if the pointer is near the
/// screen edge or actually outside it.
///
/// The frontend is free to scale/resize this screen as it sees fit, however,
/// (X, Y) = `(-0x7fff, -0x7fff)` will correspond to the top-left pixel of the
/// game image, etc.
///
/// To check if the pointer coordinates are valid (e.g. a touch display actually
/// being touched), [`RETRO_DEVICE_ID_POINTER_PRESSED`] returns 1 or 0.
///
/// If using a mouse on a desktop, [`RETRO_DEVICE_ID_POINTER_PRESSED`] will
/// usually correspond to the left mouse button, but this is a frontend
/// decision. [`RETRO_DEVICE_ID_POINTER_PRESSED`] will only return 1 if the
/// pointer is inside the game screen.
///
/// For multi-touch, the index variable can be used to successively query more
/// presses. If index = 0 returns true for `PRESSED`, coordinates can be
/// extracted with `X`, `Y` for index = 0. One can then query `PRESSED`, `X`,
/// `Y` with index = 1, and so on. Eventually `PRESSED` will return false for an
/// index. No further presses are registered at this point.
///
/// # See
///
/// - [`RETRO_DEVICE_MOUSE`]
/// - [`RETRO_DEVICE_ID_POINTER_X`]
/// - [`RETRO_DEVICE_ID_POINTER_Y`]
/// - [`RETRO_DEVICE_ID_POINTER_PRESSED`]
pub const RETRO_DEVICE_POINTER: unsigned = 6;

/* RetroPad Input
 *
 * Digital buttons for the RetroPad.
 *
 * Button placement is comparable to that of a SNES controller, combined with
 * the shoulder buttons of a PlayStation controller. These values can also be
 * used for the `id` field of `RETRO_DEVICE_INDEX_ANALOG_BUTTON` to represent
 * analog buttons (usually shoulder triggers).
 */

/// The equivalent of the SNES controller's south face button.
pub const RETRO_DEVICE_ID_JOYPAD_B: unsigned = 0;

/// The equivalent of the SNES controller's west face button.
pub const RETRO_DEVICE_ID_JOYPAD_Y: unsigned = 1;

/// The equivalent of the SNES controller's left-center button.
pub const RETRO_DEVICE_ID_JOYPAD_SELECT: unsigned = 2;

/// The equivalent of the SNES controller's right-center button.
pub const RETRO_DEVICE_ID_JOYPAD_START: unsigned = 3;

/// Up on the RetroPad's D-pad.
pub const RETRO_DEVICE_ID_JOYPAD_UP: unsigned = 4;

/// Down on the RetroPad's D-pad.
pub const RETRO_DEVICE_ID_JOYPAD_DOWN: unsigned = 5;

/// Left on the RetroPad's D-pad.
pub const RETRO_DEVICE_ID_JOYPAD_LEFT: unsigned = 6;

/// Right on the RetroPad's D-pad.
pub const RETRO_DEVICE_ID_JOYPAD_RIGHT: unsigned = 7;

/// The equivalent of the SNES controller's east face button.
pub const RETRO_DEVICE_ID_JOYPAD_A: unsigned = 8;

/// The equivalent of the SNES controller's north face button.
pub const RETRO_DEVICE_ID_JOYPAD_X: unsigned = 9;

/// The equivalent of the SNES controller's left shoulder button.
pub const RETRO_DEVICE_ID_JOYPAD_L: unsigned = 10;

/// The equivalent of the SNES controller's right shoulder button.
pub const RETRO_DEVICE_ID_JOYPAD_R: unsigned = 11;

/// The equivalent of the PlayStation's rear left shoulder button.
pub const RETRO_DEVICE_ID_JOYPAD_L2: unsigned = 12;

/// The equivalent of the PlayStation's rear right shoulder button.
pub const RETRO_DEVICE_ID_JOYPAD_R2: unsigned = 13;

/// The equivalent of the PlayStation's left analog stick button, although the
/// actual button need not be in this position.
pub const RETRO_DEVICE_ID_JOYPAD_L3: unsigned = 14;

/// The equivalent of the PlayStation's right analog stick button, although the
/// actual button need not be in this position.
pub const RETRO_DEVICE_ID_JOYPAD_R3: unsigned = 15;

/// Represents a bitmask that describes the state of all
/// `RETRO_DEVICE_ID_JOYPAD` button constants, rather than the state of a single
/// button.
///
/// # See
///
/// - [`RETRO_ENVIRONMENT_GET_INPUT_BITMASKS`]
/// - [`RETRO_DEVICE_JOYPAD`]
pub const RETRO_DEVICE_ID_JOYPAD_MASK: unsigned = 256;

/* Analog RetroPad Input */

/* Index / Id values for ANALOG device. */
pub const RETRO_DEVICE_INDEX_ANALOG_LEFT: unsigned        = 0;
pub const RETRO_DEVICE_INDEX_ANALOG_RIGHT: unsigned       = 1;
pub const RETRO_DEVICE_INDEX_ANALOG_BUTTON: unsigned      = 2;
pub const RETRO_DEVICE_ID_ANALOG_X: unsigned              = 0;
pub const RETRO_DEVICE_ID_ANALOG_Y: unsigned              = 1;

/* ID values for MOUSE. */
pub const RETRO_DEVICE_ID_MOUSE_X: unsigned               = 0;
pub const RETRO_DEVICE_ID_MOUSE_Y: unsigned               = 1;
pub const RETRO_DEVICE_ID_MOUSE_LEFT: unsigned            = 2;
pub const RETRO_DEVICE_ID_MOUSE_RIGHT: unsigned           = 3;
pub const RETRO_DEVICE_ID_MOUSE_WHEELUP: unsigned         = 4;
pub const RETRO_DEVICE_ID_MOUSE_WHEELDOWN: unsigned       = 5;
pub const RETRO_DEVICE_ID_MOUSE_MIDDLE: unsigned          = 6;
pub const RETRO_DEVICE_ID_MOUSE_HORIZ_WHEELUP: unsigned   = 7;
pub const RETRO_DEVICE_ID_MOUSE_HORIZ_WHEELDOWN: unsigned = 8;
pub const RETRO_DEVICE_ID_MOUSE_BUTTON_4: unsigned        = 9;
pub const RETRO_DEVICE_ID_MOUSE_BUTTON_5: unsigned        = 10;

/* ID values for LIGHTGUN. */
/// Absolute position
pub const RETRO_DEVICE_ID_LIGHTGUN_SCREEN_X: unsigned     = 13;
/// Absolute position
pub const RETRO_DEVICE_ID_LIGHTGUN_SCREEN_Y: unsigned     = 14;
/// Status check
///
/// Indicates if lightgun points off the screen or near the edge
pub const RETRO_DEVICE_ID_LIGHTGUN_IS_OFFSCREEN: unsigned = 15;
pub const RETRO_DEVICE_ID_LIGHTGUN_TRIGGER: unsigned      = 2;
/// Forced off-screen shot
pub const RETRO_DEVICE_ID_LIGHTGUN_RELOAD: unsigned       = 16;
pub const RETRO_DEVICE_ID_LIGHTGUN_AUX_A: unsigned        = 3;
pub const RETRO_DEVICE_ID_LIGHTGUN_AUX_B: unsigned        = 4;
pub const RETRO_DEVICE_ID_LIGHTGUN_START: unsigned        = 6;
pub const RETRO_DEVICE_ID_LIGHTGUN_SELECT: unsigned       = 7;
pub const RETRO_DEVICE_ID_LIGHTGUN_AUX_C: unsigned        = 8;
pub const RETRO_DEVICE_ID_LIGHTGUN_DPAD_UP: unsigned      = 9;
pub const RETRO_DEVICE_ID_LIGHTGUN_DPAD_DOWN: unsigned    = 10;
pub const RETRO_DEVICE_ID_LIGHTGUN_DPAD_LEFT: unsigned    = 11;
pub const RETRO_DEVICE_ID_LIGHTGUN_DPAD_RIGHT: unsigned   = 12;
/// Relative position
#[deprecated]
pub const RETRO_DEVICE_ID_LIGHTGUN_X: unsigned            = 0;
/// Relative position
#[deprecated]
pub const RETRO_DEVICE_ID_LIGHTGUN_Y: unsigned            = 1;
/// Use Aux:A instead
#[deprecated]
pub const RETRO_DEVICE_ID_LIGHTGUN_CURSOR: unsigned       = 3;
/// Use Aux:B instead
#[deprecated]
pub const RETRO_DEVICE_ID_LIGHTGUN_TURBO: unsigned        = 4;
/// Use Start instead
#[deprecated]
pub const RETRO_DEVICE_ID_LIGHTGUN_PAUSE: unsigned        = 5;

/* ID values for POINTER. */
pub const RETRO_DEVICE_ID_POINTER_X: unsigned             = 0;
pub const RETRO_DEVICE_ID_POINTER_Y: unsigned             = 1;
pub const RETRO_DEVICE_ID_POINTER_PRESSED: unsigned       = 2;
pub const RETRO_DEVICE_ID_POINTER_COUNT: unsigned         = 3;
/// Indicates if pointer is off the screen or near the edge
pub const RETRO_DEVICE_ID_POINTER_IS_OFFSCREEN: unsigned  = 15;
