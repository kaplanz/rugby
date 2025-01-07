//! Environment callbacks.

use super::unsigned;
// documentation uses
#[allow(unused_imports)]
use crate::*;

/// This bit indicates that the associated environment call is experimental, and
/// may be changed or removed in the future.
///
/// Frontends should mask out this bit before handling the environment call.
pub const RETRO_ENVIRONMENT_EXPERIMENTAL: unsigned = 0x10000;

/// Frontend-internal environment callbacks should include this bit.
pub const RETRO_ENVIRONMENT_PRIVATE: unsigned = 0x20000;

/* Environment commands. */

/// Requests the frontend to set the screen rotation.
///
/// # Parameters
///
/// - input `const unsigned *data`: Valid values are 0, 1, 2, and 3. These
///   numbers respectively set the screen rotation to 0, 90, 180, and 270
///   degrees counter-clockwise.
///
/// # Returns
///
/// `true` if the screen rotation was set successfully.
pub const RETRO_ENVIRONMENT_SET_ROTATION: unsigned = 1;

/// Queries whether the core should use overscan or not.
///
/// # Parameters
///
/// - output `bool *data`: Set to `true` if the core should use overscan,
///   `false` if it should be cropped away.
///
/// # Returns
///
/// `true` if the environment call is available. Does *not* indicate whether
/// overscan should be used.
///
/// # Deprecated
///
/// As of 2019 this callback is considered deprecated in favor of using core
/// options to manage overscan in a more nuanced, core-specific way.
#[deprecated]
pub const RETRO_ENVIRONMENT_GET_OVERSCAN: unsigned = 2;

/// Queries whether the frontend supports frame duping, in the form of passing
/// `NULL` to the video frame callback.
///
/// # Parameters
///
/// - output `bool *data`: Set to `true` if the frontend supports frame duping.
///
/// # Returns
///
/// `true` if the environment call is available.
///
/// # See
///
/// - [`retro_video_refresh_t`]
pub const RETRO_ENVIRONMENT_GET_CAN_DUPE: unsigned = 3;

// Environ 4, 5 are no longer supported (GET_VARIABLE / SET_VARIABLES), and
// reserved to avoid possible ABI clash.

/// Displays a user-facing message for a short time.
///
/// Use this callback to convey important status messages, such as errors or the
/// result of long-running operations. For trivial messages or logging, use
/// `RETRO_ENVIRONMENT_GET_LOG_INTERFACE` or `stderr`.
///
/// ```c
/// void set_message_example(void)
/// {
///    struct retro_message msg;
///    msg.frames = 60 * 5; // 5 seconds
///    msg.msg = "Hello world!";
///
///    environ_cb(RETRO_ENVIRONMENT_SET_MESSAGE, &msg);
/// }
/// ```
///
/// # Deprecated
///
/// Prefer using `RETRO_ENVIRONMENT_SET_MESSAGE_EXT` for new code, as it offers
/// more features. Only use this environment call for compatibility with older
/// cores or frontends.
///
/// # Parameters
///
/// - input `const struct retro_message *data`: Details about the message to
///   show to the user. Behavior is undefined if `NULL`.
///
/// # Returns
///
/// `true` if the environment call is available.
///
/// # See
///
/// - [`retro_message`]
/// - [`RETRO_ENVIRONMENT_GET_LOG_INTERFACE`]
/// - [`RETRO_ENVIRONMENT_SET_MESSAGE_EXT`]
/// - [`RETRO_ENVIRONMENT_SET_MESSAGE`]
/// - [`RETRO_ENVIRONMENT_GET_MESSAGE_INTERFACE_VERSION`]
///
/// # Note
///
/// The frontend must make its own copy of the message and the underlying
/// string.
#[deprecated]
pub const RETRO_ENVIRONMENT_SET_MESSAGE: unsigned = 6;

/// Requests the frontend to shutdown the core. Should only be used if the core
/// can exit on its own, such as from a menu item in a game or an emulated
/// power-off in an emulator.
///
/// # Parameters
///
/// - `data`: Ignored.
///
/// # Returns
///
/// `true` if the environment call is available.
pub const RETRO_ENVIRONMENT_SHUTDOWN: unsigned = 7;

/// Gives a hint to the frontend of how demanding this core is on the system.
/// For example, reporting a level of 2 means that this implementation should
/// run decently on frontends of level 2 and above.
///
/// It can be used by the frontend to potentially warn about too demanding
/// implementations.
///
/// The levels are "floating".
///
/// This function can be called on a per-game basis, as a core may have
/// different demands for different games or settings. If called, it should be
/// called in `retro_load_game`.
///
/// # Parameters
///
/// - input `const unsigned *data`
pub const RETRO_ENVIRONMENT_SET_PERFORMANCE_LEVEL: unsigned = 8;

/// Returns the path to the frontend's system directory, which can be used to
/// store system-specific configuration such as BIOS files or cached data.
///
/// # Parameters
///
/// - output `const char** data`: Pointer to the `char`* in which the system
///   directory will be saved. The string is managed by the frontend and must
///   not be modified or freed by the core. May be `NULL` if no system directory
///   is defined, in which case the core should find an alternative directory.
///
/// # Returns
///
/// `true` if the environment call is available, even if the value returned in
/// `data` is `NULL`.
///
/// # Note
///
/// Historically, some cores would use this folder for save data such as memory
/// cards or SRAM. This is now discouraged in favor of
/// `RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY`.
///
/// # See
///
/// - [`RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY`]
pub const RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY: unsigned = 9;

/// Sets the internal pixel format used by the frontend for rendering. The
/// default pixel format is `RETRO_PIXEL_FORMAT_0RGB1555` for compatibility
/// reasons, although it's considered deprecated and shouldn't be used by new
/// code.
///
/// # Parameters
///
/// - input `const enum retro_pixel_format *data`: Pointer to the pixel format
///   to use.
///
/// # Returns
///
/// `true` if the pixel format was set successfully, `false` if it's not
/// supported or this callback is unavailable.
///
/// # Note
///
/// This function should be called inside `retro_load_game` or
/// `retro_get_system_av_info`.
///
/// # See
///
/// - [`retro_pixel_format`]
pub const RETRO_ENVIRONMENT_SET_PIXEL_FORMAT: unsigned = 10;

/// Sets an array of input descriptors for the frontend to present to the user
/// for configuring the core's controls.
///
/// This function can be called at any time, preferably early in the core's life
/// cycle. Ideally, no later than `retro_load_game`.
///
/// # Parameters
///
/// - input `const struct retro_input_descriptor *data`: An array of input
///   descriptors terminated by one whose `retro_input_descriptor::description`
///   field is set to `NULL`. Behavior is undefined if `NULL`.
///
/// # Returns
///
/// `true` if the environment call is recognized.
///
/// # See
///
/// - [`retro_input_descriptor`]
pub const RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS: unsigned = 11;

/// Sets a callback function used to notify the core about keyboard events. This
/// should only be used for cores that specifically need keyboard input, such as
/// for home computer emulators or games with text entry.
///
/// # Parameters
///
/// - input `const struct retro_keyboard_callback *data`: Pointer to the
///   callback function. Behavior is undefined if `NULL`.
///
/// # Returns
///
/// `true` if the environment call is recognized.
///
/// # See
///
/// - [`retro_keyboard_callback`]
/// - [`retro_key`]
pub const RETRO_ENVIRONMENT_SET_KEYBOARD_CALLBACK: unsigned = 12;

/// Sets an interface that the frontend can use to insert and remove disks from
/// the emulated console's disk drive. Can be used for optical disks, floppy
/// disks, or any other game storage medium that can be swapped at runtime.
///
/// This is intended for multi-disk games that expect the player to manually
/// swap disks at certain points in the game.
///
/// # Deprecated
///
/// Prefer using `RETRO_ENVIRONMENT_SET_DISK_CONTROL_EXT_INTERFACE` over this
/// environment call, as it supports additional features. Only use this callback
/// to maintain compatibility with older cores or frontends.
///
/// # Parameters
///
/// - input `const struct retro_disk_control_callback *data`: Pointer to the
///   callback functions to use. May be `NULL`, in which case the existing disk
///   callback is deregistered.
///
/// # Returns
///
/// `true` if this environment call is available,
/// even if `data` is `NULL`.
///
/// # See
///
/// - [`retro_disk_control_callback`]
/// - [`RETRO_ENVIRONMENT_SET_DISK_CONTROL_EXT_INTERFACE`]
#[deprecated]
pub const RETRO_ENVIRONMENT_SET_DISK_CONTROL_INTERFACE: unsigned = 13;

/// Requests that a frontend enable a particular hardware rendering API.
///
/// If successful, the frontend will create a context (and other related
/// resources) that the core can use for rendering. The framebuffer will be at
/// least as large as the maximum dimensions provided in
/// `retro_get_system_av_info`.
///
/// # Parameters
///
/// - inout `struct retro_hw_render_callback *data`: Pointer to the hardware
///   render callback struct. Used to define callbacks for the
///   hardware-rendering life cycle, as well as to request a particular
///   rendering API.
///
/// # Returns
///
/// `true` if the environment call is recognized and the requested rendering API
/// is supported. `false` if `data` is `NULL` or the frontend can't provide the
/// requested rendering API.
///
/// # See
///
/// - [`retro_hw_render_callback`]
/// - [`retro_video_refresh_t`]
/// - [`RETRO_ENVIRONMENT_GET_PREFERRED_HW_RENDER`]
///
/// # Note
///
/// Should be called in `retro_load_game`.
///
/// # Note
///
/// If HW rendering is used, pass only `RETRO_HW_FRAME_BUFFER_VALID` or `NULL`
/// to `retro_video_refresh_t`.
pub const RETRO_ENVIRONMENT_SET_HW_RENDER: unsigned = 14;

/// Retrieves a core option's value from the frontend. `retro_variable::key`
/// should be set to an option key that was previously set in
/// `RETRO_ENVIRONMENT_SET_VARIABLES` (or a similar environment call).
///
/// # Parameters
///
/// - inout `struct retro_variable *data`: Pointer to a single `retro_variable`
///   struct. See the documentation for `retro_variable` for details on which
///   fields are set by the frontend or core. May be `NULL`.
///
/// # Returns
///
/// `true` if the environment call is available, even if `data` is `NULL` or the
/// key it specifies is not found.
///
/// # Note
///
/// Passing `NULL` in to `data` can be useful to test for support of this
/// environment call without looking up any variables.
///
/// # See
///
/// - [`retro_variable`]
/// - [`RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2`]
/// - [`RETRO_ENVIRONMENT_GET_VARIABLE_UPDATE`]
pub const RETRO_ENVIRONMENT_GET_VARIABLE: unsigned = 15;

/// Notifies the frontend of the core's available options.
///
/// The core may check these options later using
/// `RETRO_ENVIRONMENT_GET_VARIABLE`. The frontend may also present these
/// options to the user in its own configuration UI.
///
/// This should be called the first time as early as possible, ideally in
/// `retro_set_environment`. The core may later call this function again to
/// communicate updated options to the frontend, but the number of core options
/// must not change.
///
/// Here's an example that sets two options.
///
/// ```c
/// void set_variables_example(void)
/// {
///    struct retro_variable options[] = {
///        { "foo_speedhack", "Speed hack; false|true" }, // false by default
///        { "foo_displayscale", "Display scale factor; 1|2|3|4" }, // 1 by default
///        { NULL, NULL },
///    };
///
///    environ_cb(RETRO_ENVIRONMENT_SET_VARIABLES, &options);
/// }
/// ```
///
/// The possible values will generally be displayed and stored as-is by the
/// frontend.
///
/// # Deprecated
///
/// Prefer using `RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2` for new code, as it
/// offers more features such as categories and translation. Only use this
/// environment call to maintain compatibility with older frontends or cores.
///
/// # Note
///
/// Keep the available options (and their possible values) as low as possible;
/// it should be feasible to cycle through them without a keyboard.
///
/// # Parameters
///
/// - input `const struct retro_variable *data`: Pointer to an array of
///   `retro_variable` structs that define available core options, terminated by
///   a `{ NULL, NULL }` element. The frontend must maintain its own copy of
///   this array.
///
/// # Returns
///
/// `true` if the environment call is available, even if `data` is `NULL`.
///
/// # See
///
/// - [`retro_variable`]
/// - [`RETRO_ENVIRONMENT_GET_VARIABLE`]
/// - [`RETRO_ENVIRONMENT_GET_VARIABLE_UPDATE`]
/// - [`RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2`]
#[deprecated]
pub const RETRO_ENVIRONMENT_SET_VARIABLES: unsigned = 16;

/// Queries whether at least one core option was updated by the frontend since
/// the last call to [`RETRO_ENVIRONMENT_GET_VARIABLE`]. This typically means
/// that the user opened the core options menu and made some changes.
///
/// Cores usually call this each frame before the core's main emulation logic.
/// Specific options can then be queried with
/// [`RETRO_ENVIRONMENT_GET_VARIABLE`].
///
/// # Parameters
///
/// - output `bool *data`: Set to `true` if at least one core option was updated
///   since the last call to [`RETRO_ENVIRONMENT_GET_VARIABLE`]. Behavior is
///   undefined if this pointer is `NULL`.
///
/// # Returns
///
/// `true` if the environment call is available.
///
/// # See
///
/// - [`RETRO_ENVIRONMENT_GET_VARIABLE`]
/// - [`RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2`]
pub const RETRO_ENVIRONMENT_GET_VARIABLE_UPDATE: unsigned = 17;

/// Notifies the frontend that this core can run without loading any content,
/// such as when emulating a console that has built-in software. When a core is
/// loaded without content, `retro_load_game` receives an argument of `NULL`.
/// This should be called within `retro_set_environment` only.
///
/// # Parameters
///
/// - input `const bool *data`: Pointer to a single `bool` that indicates
///   whether this frontend can run without content. Can point to a value of
///   `false` but this isn't necessary, as contentless support is opt-in. The
///   behavior is undefined if `data` is `NULL`.
///
/// # Returns
///
/// `true` if the environment call is available.
///
/// # See
///
/// - [`retro_load_game`]
pub const RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME: unsigned = 18;

/// Retrieves the absolute path from which this core was loaded. Useful when
/// loading assets from paths relative to the core, as is sometimes the case
/// when using `RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME`.
///
/// # Parameters
///
/// - output `const char ** data`: Pointer to a string in which the core's path
///   will be saved. The string is managed by the frontend and must not be
///   modified or freed by the core. May be `NULL` if the core is statically
///   linked to the frontend or if the core's path otherwise cannot be
///   determined. Behavior is undefined if `data` is `NULL`.
///
/// # Returns
///
/// `true` if the environment call is available.
pub const RETRO_ENVIRONMENT_GET_LIBRETRO_PATH: unsigned = 19;

// Environment call 20 was an obsolete version of SET_AUDIO_CALLBACK. It was not
// used by any known core at the time, and was removed from the API. The number
// 20 is reserved to prevent ABI clashes.

/// Sets a callback that notifies the core of how much time has passed since the
/// last iteration of `retro_run`.
///
/// If the frontend is not running the core in real time (e.g. it's
/// frame-stepping or running in slow motion), then the reference value will be
/// provided to the callback instead.
///
/// # Parameters
///
/// - input `const struct retro_frame_time_callback *data`: Pointer to a single
///   `retro_frame_time_callback` struct. Behavior is undefined if `data` is
///   `NULL`.
///
/// # Returns
///
/// `true` if the environment call is available.
///
/// # Note
///
/// Frontends may disable this environment call in certain situations. It will
/// return `false` in those cases.
///
/// # See
///
/// - [`retro_frame_time_callback`]
pub const RETRO_ENVIRONMENT_SET_FRAME_TIME_CALLBACK: unsigned = 21;

/// Registers a set of functions that the frontend can use to tell the core it's
/// ready for audio output.
///
/// It is intended for games that feature asynchronous audio. It should not be
/// used for emulators unless their audio is asynchronous.
///
///
/// The callback only notifies about writability; the libretro core still has to
/// call the normal audio callbacks to write audio. The audio callbacks must be
/// called from within the notification callback. The amount of audio data to
/// write is up to the core. Generally, the audio callback will be called
/// continuously in a loop.
///
/// A frontend may disable this callback in certain situations. The core must be
/// able to render audio with the "normal" interface.
///
/// # Parameters
///
/// - input `const struct retro_audio_callback *data`: Pointer to a set of
///   functions that the frontend will call to notify the core when it's ready
///   to receive audio data. May be `NULL`, in which case the frontend will
///   return whether this environment callback is available.
///
/// # Returns
///
/// `true` if this environment call is available, even if `data` is `NULL`.
///
/// # Warning
///
/// The provided callbacks can be invoked from any thread, so their
/// implementations *must* be thread-safe.
///
/// # Note
///
/// If a core uses this callback, it should also use
/// `RETRO_ENVIRONMENT_SET_FRAME_TIME_CALLBACK`.
///
/// # See
///
/// - [`retro_audio_callback`]
/// - [`retro_audio_sample_t`]
/// - [`retro_audio_sample_batch_t`]
/// - [`RETRO_ENVIRONMENT_SET_FRAME_TIME_CALLBACK`]
pub const RETRO_ENVIRONMENT_SET_AUDIO_CALLBACK: unsigned = 22;

/// Gets an interface that a core can use to access a controller's rumble
/// motors.
///
/// The interface supports two independently-controlled motors, one strong and
/// one weak.
///
/// Should be called from either `retro_init` or `retro_load_game`, but not from
/// `retro_set_environment`.
///
/// # Parameters
///
/// - output `struct retro_rumble_interface *data`: Pointer to the interface
///   struct. Behavior is undefined if `NULL`.
///
/// # Returns
///
/// `true` if the environment call is available, even if the current device
/// doesn't support vibration.
///
/// # See
///
/// - [`retro_rumble_interface`]
pub const RETRO_ENVIRONMENT_GET_RUMBLE_INTERFACE: unsigned = 23;

/// Returns the frontend's supported input device types.
///
/// The supported device types are returned as a bitmask, with each value of
/// [`RETRO_DEVICE`] corresponding to a bit.
///
/// Should only be called in `retro_run`.
///
/// ```c
/// const REQUIRED_DEVICES ((1 << RETRO_DEVICE_JOYPAD) | (1 << RETRO_DEVICE_ANALOG))
/// void get_input_device_capabilities_example(void)
/// {
///    uint64_t capabilities;
///    environ_cb(RETRO_ENVIRONMENT_GET_INPUT_DEVICE_CAPABILITIES, &capabilities);
///    if ((capabilities & REQUIRED_DEVICES) == REQUIRED_DEVICES)
///      printf("Joypad and analog device types are supported");
/// }
/// ```
///
/// # Parameters
///
/// - output `uint64_t *data`: Pointer to a bitmask of supported input device
///   types. If the frontend supports a particular `RETRO_DEVICE`_* type, then
///   the bit `(1 << RETRO_DEVICE_*)` will be set.
///
/// Each bit represents a `RETRO_DEVICE` constant, e.g. bit 1 represents
/// `RETRO_DEVICE_JOYPAD`, bit 2 represents `RETRO_DEVICE_MOUSE`, and so on.
///
/// Bits that do not correspond to known device types will be set to zero and
/// are reserved for future use.
///
/// Behavior is undefined if `NULL`.
///
/// # Returns
///
/// `true` if the environment call is available.
///
/// # Note
///
/// If the frontend supports multiple input drivers, availability of this
/// environment call (and the reported capabilities) may depend on the active
/// driver.
///
/// # See
///
/// - [`RETRO_DEVICE`]
pub const RETRO_ENVIRONMENT_GET_INPUT_DEVICE_CAPABILITIES: unsigned = 24;

/// Returns an interface that the core can use to access and configure available
/// sensors, such as an accelerometer or gyroscope.
///
/// # Parameters
///
/// - output `struct retro_sensor_interface *data`: Pointer to the sensor
///   interface that the frontend will populate. Behavior is undefined if is
///   `NULL`.
///
/// # Returns
///
/// `true` if the environment call is available, even if the device doesn't have
/// any supported sensors.
///
/// # See
///
/// - [`retro_sensor_interface`]
/// - [`retro_sensor_action`]
/// - [`RETRO_SENSOR`]
pub const RETRO_ENVIRONMENT_GET_SENSOR_INTERFACE: unsigned = 25 | RETRO_ENVIRONMENT_EXPERIMENTAL;

/// Gets an interface to the device's video camera.
///
/// The frontend delivers new video frames via a user-defined callback that runs
/// in the same thread as `retro_run`. Should be called in `retro_load_game`.
///
/// # Parameters
///
/// - inout `struct retro_camera_callback *data`: Pointer to the camera driver
///   interface. Some fields in the struct must be filled in by the core, others
///   are provided by the frontend. Behavior is undefined if `NULL`.
///
/// # Returns
///
/// `true` if this environment call is available, even if an actual camera
/// isn't.
///
/// # Note
///
/// This API only supports one video camera at a time. If the device provides
/// multiple cameras (e.g. inner/outer cameras on a phone), the frontend will
/// choose one to use.
///
/// # See
///
/// - [`retro_camera_callback`]
/// - [`RETRO_ENVIRONMENT_SET_HW_RENDER`]
pub const RETRO_ENVIRONMENT_GET_CAMERA_INTERFACE: unsigned = 26 | RETRO_ENVIRONMENT_EXPERIMENTAL;

/// Gets an interface that the core can use for cross-platform logging. Certain
/// platforms don't have a console or `stderr`, or they have their own preferred
/// logging methods. The frontend itself may also display log output.
///
/// # Attention
///
/// This should not be used for information that the player must immediately
/// see, such as major errors or warnings. In most cases, this is best for
/// information that will help you (the developer) identify problems when
/// debugging or providing support. Unless a core or frontend is intended for
/// advanced users, the player might not check (or even know about) their logs.
///
/// # Parameters
///
/// - output `struct retro_log_callback *data`: Pointer to the callback where
///   the function pointer will be saved. Behavior is undefined if `data` is
///   `NULL`.
///
/// # Returns
///
/// `true` if the environment call is available.
///
/// # See
///
/// - [`retro_log_callback`]
///
/// # Note
///
/// Cores can fall back to `stderr` if this interface is not available.
pub const RETRO_ENVIRONMENT_GET_LOG_INTERFACE: unsigned = 27;

/// Returns an interface that the core can use for profiling code and to access
/// performance-related information.
///
/// This callback supports performance counters, a high-resolution timer, and
/// listing available CPU features (mostly SIMD instructions).
///
/// # Parameters
///
/// - output `struct retro_perf_callback *data`: Pointer to the callback
///   interface. Behavior is undefined if `NULL`.
///
/// # Returns
///
/// `true` if the environment call is available.
///
/// # See
///
/// - [`retro_perf_callback`]
pub const RETRO_ENVIRONMENT_GET_PERF_INTERFACE: unsigned = 28;

/// Returns an interface that the core can use to retrieve the device's
/// location, including its current latitude and longitude.
///
/// # Parameters
///
/// - output `struct retro_location_callback *data`: Pointer to the callback
///   interface. Behavior is undefined if `NULL`.
///
/// # Returns
///
/// `true` if the environment call is available, even if there's no location
/// information available.
///
/// # See
///
/// - [`retro_location_callback`]
pub const RETRO_ENVIRONMENT_GET_LOCATION_INTERFACE: unsigned = 29;

/// # Deprecated
///
/// An obsolete alias to `RETRO_ENVIRONMENT_GET_CORE_ASSETS_DIRECTORY` kept for
/// compatibility.
///
/// # See
///
/// - [`RETRO_ENVIRONMENT_GET_CORE_ASSETS_DIRECTORY`]
#[deprecated]
pub const RETRO_ENVIRONMENT_GET_CONTENT_DIRECTORY: unsigned = 30;

/// Returns the frontend's "core assets" directory, which can be used to store
/// assets that the core needs such as art assets or level data.
///
/// # Parameters
///
/// - output `const char ** data`: Pointer to a string in which the core assets
///   directory will be saved. This string is managed by the frontend and must not
///   be modified or freed by the core. May be `NULL` if no core assets directory
///   is defined, in which case the core should find an alternative directory.
///   Behavior is undefined if `data` is `NULL`.
///
/// # Returns
///
/// `true` if the environment call is available, even if the value returned in
/// `data` is `NULL`.
pub const RETRO_ENVIRONMENT_GET_CORE_ASSETS_DIRECTORY: unsigned = 30;

/// Returns the frontend's save data directory, if available. This directory
/// should be used to store game-specific save data, including memory card
/// images.
///
/// Although libretro provides an interface for cores to expose SRAM to the
/// frontend, not all cores can support it correctly. In this case, cores should
/// use this environment callback to save their game data to disk manually.
///
/// Cores that use this environment callback should flush their save data to
/// disk periodically and when unloading.
///
/// # Parameters
///
/// - output `const char ** data`: Pointer to the string in which the save data
///   directory will be saved. This string is managed by the frontend and must
///   not be modified or freed by the core. May return `NULL` if no save data
///   directory is defined. Behavior is undefined if `data` is `NULL`.
///
/// # Returns
///
/// `true` if the environment call is available, even if the value returned in
/// `data` is `NULL`.
///
/// # Note
///
/// Early libretro cores used `RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY` for save
/// data. This is still supported for backwards compatibility, but new cores
/// should use this environment call instead.
/// `RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY` should be used for game-agnostic
/// data such as BIOS files or core-specific configuration.
///
/// # Note
///
/// The returned directory may or may not be the same as the one used for
/// `retro_get_memory_data`.
///
/// # See
///
/// - [`retro_get_memory_data`]
/// - [`RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY`]
pub const RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY: unsigned = 31;

/// Sets new video and audio parameters for the core. This can only be called
/// from within `retro_run`.
///
/// This environment call may entail a full reinitialization of the frontend's
/// audio/video drivers, hence it should *only* be used if the core needs to
/// make drastic changes to audio/video parameters.
///
/// This environment call should *not* be used when:
///
/// - Changing the emulated system's internal resolution, within the limits
///   defined by the existing values of `max_width` and `max_height`. Use
///   `RETRO_ENVIRONMENT_SET_GEOMETRY` instead, and adjust
///   `retro_get_system_av_info` to account for supported scale factors and
///   screen layouts when computing `max_width` and `max_height`. Only use this
///   environment call if `max_width` or `max_height` needs to increase.
/// - Adjusting the screen's aspect ratio, e.g. when changing the layout of the
///   screen(s). Use `RETRO_ENVIRONMENT_SET_GEOMETRY` or
///   `RETRO_ENVIRONMENT_SET_ROTATION` instead.
///
/// The frontend will reinitialize its audio and video drivers within this
/// callback; after that happens, audio and video callbacks will target the
/// newly-initialized driver, even within the same `retro_run` call.
///
/// This callback makes it possible to support configurable resolutions while
/// avoiding the need to compute the "worst case" values of `max_width` and
/// `max_height`.
///
/// # Parameters
///
/// - input `const struct retro_system_av_info *data`: Pointer to the new video
///   and audio parameters that the frontend should adopt.
///
/// # Returns
///
/// `true` if the environment call is available and the new `av_info` struct was
/// accepted. `false` if the environment call is unavailable or `data` is
/// `NULL`.
///
/// # See
///
/// - [`retro_system_av_info`]
/// - [`RETRO_ENVIRONMENT_SET_GEOMETRY`]
pub const RETRO_ENVIRONMENT_SET_SYSTEM_AV_INFO: unsigned = 32;

/// Provides an interface that a frontend can use to get function pointers from
/// the core.
///
/// This allows cores to define their own extensions to the libretro API, or to
/// expose implementations of a frontend's libretro extensions.
///
/// # Parameters
///
/// - input `const struct retro_get_proc_address_interface *data`: Pointer to
///   the interface that the frontend can use to get function pointers from the
///   core. The frontend must maintain its own copy of this interface.
///
/// # Returns
///
/// `true` if the environment call is available and the returned interface was
/// accepted.
///
/// # Note
///
/// The provided interface may be called at any time, even before this
/// environment call returns.
///
/// # Note
///
/// Extensions should be prefixed with the name of the frontend or core that
/// defines them. For example, a frontend named "foo" that defines a debugging
/// extension should expect the core to define functions prefixed with
/// "`foo_debug`_".
///
/// # Warning
///
/// If a core wants to use this environment call, it *must* do so from within
/// `retro_set_environment`.
///
/// # See
///
/// - [`retro_get_proc_address_interface`]
pub const RETRO_ENVIRONMENT_SET_PROC_ADDRESS_CALLBACK: unsigned = 33;

/// Registers a core's ability to handle "subsystems", which are secondary
/// platforms that augment a core's primary emulated hardware.
///
/// A core doesn't need to emulate a secondary platform in order to use it as a
/// subsystem; as long as it can load a secondary file for some practical use,
/// then this environment call is most likely suitable.
///
/// Possible use cases of a subsystem include:
///
/// - Installing software onto an emulated console's internal storage, such as
///   the Nintendo `DSi`.
/// - Emulating accessories that are used to support another console's games,
///   such as the Super Game Boy or the N64 Transfer Pak.
/// - Inserting a secondary ROM into a console that features multiple cartridge
///   ports, such as the Nintendo DS's Slot-2.
/// - Loading a save data file created and used by another core.
///
/// Cores should *not* use subsystems for:
///
/// - Emulators that support multiple "primary" platforms, such as a Game
///   Boy/Game Boy Advance core or a Sega Genesis/Sega CD/32X core. Use
///   `retro_system_content_info_override`, `retro_system_info`, and/or runtime
///   detection instead.
/// - Selecting different memory card images. Use dynamically-populated core
///   options instead.
/// - Different variants of a single console, such the Game Boy vs. the Game Boy
///   Color. Use core options or runtime detection instead.
/// - Games that span multiple disks. Use
///   `RETRO_ENVIRONMENT_SET_DISK_CONTROL_EXT_INTERFACE` and m3u-formatted
///   playlists instead.
/// - Console system files (BIOS, firmware, etc.). Use
///   `RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY` and a common naming convention
///   instead.
///
/// When the frontend loads a game via a subsystem, it must call
/// `retro_load_game_special` instead of `retro_load_game`.
///
/// # Parameters
///
/// - input `const struct retro_subsystem_info *data`: Pointer to an array of
///   subsystem descriptors, terminated by a zeroed-out `retro_subsystem_info`
///   struct. The frontend should maintain its own copy of this array and the
///   strings within it. Behavior is undefined if `NULL`.
///
/// # Returns
///
/// `true` if this environment call is available.
///
/// # Note
///
/// This environment call *must* be called from within `retro_set_environment`,
/// as frontends may need the registered information before loading a game.
///
/// # See
///
/// - [`retro_subsystem_info`]
/// - [`retro_load_game_special`]
pub const RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO: unsigned = 34;

/// Declares one or more types of controllers supported by this core. The
/// frontend may then allow the player to select one of these controllers in its
/// menu.
///
/// Many consoles had controllers that came in different versions, were
/// extensible with peripherals, or could be held in multiple ways; this
/// environment call can be used to represent these differences and adjust the
/// core's behavior to match.
///
/// Possible use cases include:
///
/// - Supporting different classes of a single controller that supported their
///   own sets of games. For example, the SNES had two different lightguns (the
///   Super Scope and the Justifier) whose games were incompatible with each
///   other.
/// - Representing a platform's alternative controllers. For example, several
///   platforms had music/rhythm games that included controllers shaped like
///   musical instruments.
/// - Representing variants of a standard controller with additional inputs. For
///   example, numerous consoles in the 90's introduced 6-button controllers for
///   fighting games, steering wheels for racing games, or analog sticks for 3D
///   platformers.
/// - Representing add-ons for consoles or standard controllers. For example,
///   the 3DS had a Circle Pad Pro attachment that added a second analog stick.
/// - Selecting different configurations for a single controller. For example,
///   the Wii Remote could be held sideways like a traditional game pad or in one
///   hand like a wand.
/// - Providing multiple ways to simulate the experience of using a particular
///   controller. For example, the Game Boy Advance featured several games with
///   motion or light sensors in their cartridges; a core could provide
///   controller configurations that allow emulating the sensors with either
///   analog axes or with their host device's sensors.
///
/// Should be called in `retro_load_game`. The frontend must maintain its own
/// copy of the provided array, including all strings and subobjects. A core may
/// exclude certain controllers for known incompatible games.
///
/// When the frontend changes the active device for a particular port, it must
/// call `retro_set_controller_port_device` with that port's index and one of
/// the IDs defined in its `retro_controller_info::types` field.
///
/// Input ports are generally associated with different players (and the
/// frontend's UI may reflect this with "Player 1" labels), but this is not
/// required. Some games use multiple controllers for a single player, or some
/// cores may use port indexes to represent an emulated console's alternative
/// input peripherals.
///
/// # Parameters
///
/// - input `const struct retro_controller_info *data`: Pointer to an array of
///   controller types defined by this core, terminated by a zeroed-out
///   `retro_controller_info`. Each element of this array represents a
///   controller port on the emulated device. Behavior is undefined if `NULL`.
///
/// # Returns
///
/// `true` if this environment call is available.
///
/// # See
///
/// - [`retro_controller_info`]
/// - [`retro_set_controller_port_device`]
/// - [`RETRO_DEVICE_SUBCLASS`]
pub const RETRO_ENVIRONMENT_SET_CONTROLLER_INFO: unsigned = 35;

/// Notifies the frontend of the address spaces used by the core's emulated
/// hardware, and of the memory maps within these spaces.
///
/// This can be used by the frontend to provide cheats, achievements, or
/// debugging capabilities. Should only be used by emulators, as it makes little
/// sense for game engines.
///
/// # Note
///
/// Cores should also expose these address spaces through
/// `retro_get_memory_data` and `retro_get_memory_size` if applicable; this
/// environment call is not intended to replace those two functions, as the
/// emulated hardware may feature memory regions outside of its own address
/// space that are nevertheless useful for the frontend.
///
/// # Parameters
///
/// - input `const struct retro_memory_map *data`: Pointer to a single
///   memory-map listing. The frontend must maintain its own copy of this object
///   and its contents, including strings and nested objects. Behavior is
///   undefined if `NULL`.
///
/// # Returns
///
/// `true` if this environment call is available.
///
/// # See
///
/// - [`retro_memory_map`]
/// - [`retro_get_memory_data`]
/// - [`retro_memory_descriptor`]
pub const RETRO_ENVIRONMENT_SET_MEMORY_MAPS: unsigned = 36 | RETRO_ENVIRONMENT_EXPERIMENTAL;

/// Resizes the viewport without reinitializing the video driver.
///
/// Similar to `RETRO_ENVIRONMENT_SET_SYSTEM_AV_INFO`, but any changes that
/// would require video reinitialization will not be performed. Can only be
/// called from within `retro_run`.
///
/// This environment call allows a core to revise the size of the viewport at
/// will, which can be useful for emulated platforms that support dynamic
/// resolution changes or for cores that support multiple screen layouts.
///
/// A frontend must guarantee that this environment call completes in constant
/// time.
///
/// # Parameters
///
/// - input `const struct retro_game_geometry *data`: Pointer to the new video
///   parameters that the frontend should adopt.
///   `retro_game_geometry::max_width` and `retro_game_geometry::max_height`
///   will be ignored. Behavior is undefined if `data` is `NULL`.
///
/// # Returns
///
/// `true` if the environment call is available.
///
/// # See
///
/// - [`RETRO_ENVIRONMENT_SET_SYSTEM_AV_INFO`]
pub const RETRO_ENVIRONMENT_SET_GEOMETRY: unsigned = 37;

/// Returns the name of the user, if possible.
///
/// This callback is suitable for cores that offer personalization, such as
/// online facilities or user profiles on the emulated system.
///
/// # Parameters
///
/// - output `const char ** data`: Pointer to the user name string. May be
///   `NULL`, in which case the core should use a default name. The returned
///   pointer is owned by the frontend and must not be modified or freed by the
///   core. Behavior is undefined if `NULL`.
///
/// # Returns
///
/// `true` if the environment call is available, even if the frontend couldn't
/// provide a name.
pub const RETRO_ENVIRONMENT_GET_USERNAME: unsigned = 38;

/// Returns the frontend's configured language.
///
/// It can be used to localize the core's UI, or to customize the emulated
/// firmware if applicable.
///
/// # Parameters
///
/// - output `retro_language *data`: Pointer to the language identifier.
///   Behavior is undefined if `NULL`.
///
/// # Returns
///
/// `true` if the environment call is available.
///
/// # Note
///
/// The returned language may not be the same as the operating system's
/// language. Cores should fall back to the operating system's language (or to
/// English) if the environment call is unavailable or the returned language is
/// unsupported.
///
/// # See
///
/// - [`retro_language`]
/// - [`RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL`]
pub const RETRO_ENVIRONMENT_GET_LANGUAGE: unsigned = 39;

/// Returns a frontend-managed framebuffer that the core may render directly
/// into
///
/// This environment call is provided as an optimization for cores that use
/// software rendering (i.e. that don't use [`RETRO_ENVIRONMENT_SET_HW_RENDER`]
/// "a graphics hardware API"); specifically, the intended use case is to allow
/// a core to render directly into frontend-managed video memory, avoiding the
/// bandwidth use that copying a whole framebuffer from core to video memory
/// entails.
///
/// Must be called every frame if used, as this may return a different
/// framebuffer each frame (e.g. for swap chains). However, a core may render to
/// a different buffer even if this call succeeds.
///
/// # Parameters
///
/// - inout `struct retro_framebuffer *data`: Pointer to a frontend's frame
///   buffer and accompanying data. Some fields are set by the core, others are
///   set by the frontend. Only guaranteed to be valid for the duration of the
///   current `retro_run` call, and must not be used afterwards. Behavior is
///   undefined if `NULL`.
///
/// # Returns
///
/// `true` if the environment call was recognized and the framebuffer was
/// successfully returned.
///
/// # See
///
/// - [`retro_framebuffer`]
pub const RETRO_ENVIRONMENT_GET_CURRENT_SOFTWARE_FRAMEBUFFER: unsigned =
    40 | RETRO_ENVIRONMENT_EXPERIMENTAL;

/// Returns an interface for accessing the data of specific rendering APIs. Not
/// all hardware rendering APIs support or need this.
///
/// The details of these interfaces are specific to each rendering API.
///
/// # Note
///
/// `retro_hw_render_callback::context_reset` must be called by the frontend
/// before this environment call can be used. Additionally, the contents of the
/// returned interface are invalidated after
/// `retro_hw_render_callback::context_destroyed` has been called.
///
/// # Parameters
///
/// - output `const struct retro_hw_render_interface ** data`: The render
///   interface for the currently-enabled hardware rendering API, if any. The
///   frontend will store a pointer to the interface at the address provided
///   here. The returned interface is owned by the frontend and must not be
///   modified or freed by the core. Behavior is undefined if `NULL`.
///
/// # Returns
///
/// `true` if this environment call is available, the active graphics API has a
/// libretro rendering interface, and the frontend is able to return said
/// interface. `false` otherwise.
///
/// # See
///
/// - [`RETRO_ENVIRONMENT_SET_HW_RENDER`]
/// - [`retro_hw_render_interface`]
///
/// # Note
///
/// Since not every libretro-supported hardware rendering API has a
/// `retro_hw_render_interface` implementation, a result of `false` is not
/// necessarily an error.
pub const RETRO_ENVIRONMENT_GET_HW_RENDER_INTERFACE: unsigned = 41 | RETRO_ENVIRONMENT_EXPERIMENTAL;

/// Explicitly notifies the frontend of whether this core supports achievements.
///
/// The core must expose its emulated address space via `retro_get_memory_data`
/// or `RETRO_ENVIRONMENT_GET_MEMORY_MAPS`. Must be called before the first call
/// to `retro_run`.
///
/// If [`retro_get_memory_data`] returns a valid address but this environment
/// call is not used, the frontend (at its discretion) may or may not opt in the
/// core to its achievements support. whether this core is opted in to the
/// frontend's achievement support is left to the frontend's discretion.
///
/// # Parameters
///
/// - input `const bool *data`: Pointer to a single `bool` that indicates
///   whether this core supports achievements. Behavior is undefined if `data`
///   is `NULL`.
///
/// # Returns
///
/// `true` if the environment call is available.
///
/// # See
///
/// - [`RETRO_ENVIRONMENT_SET_MEMORY_MAPS`]
/// - [`retro_get_memory_data`]
pub const RETRO_ENVIRONMENT_SET_SUPPORT_ACHIEVEMENTS: unsigned =
    42 | RETRO_ENVIRONMENT_EXPERIMENTAL;

/// Defines an interface that the frontend can use to ask the core for the
/// parameters it needs for a hardware rendering context.
///
/// The exact semantics depend on [`RETRO_ENVIRONMENT_SET_HW_RENDER`] "the
/// active rendering API". Will be used some time after
/// `RETRO_ENVIRONMENT_SET_HW_RENDER` is called, but before
/// `retro_hw_render_callback::context_reset` is called.
///
/// # Parameters
///
/// - input `const struct retro_hw_render_context_negotiation_interface *data`:
///   Pointer to the context negotiation interface. Will be populated by the
///   frontend. Behavior is undefined if `NULL`.
///
/// # Returns
///
/// `true` if this environment call is supported, even if the current graphics
/// API doesn't use a context negotiation interface (in which case the argument
/// is ignored).
///
/// # See
///
/// - [`retro_hw_render_context_negotiation_interface`]
/// - [`RETRO_ENVIRONMENT_GET_HW_RENDER_CONTEXT_NEGOTIATION_INTERFACE_SUPPORT`]
/// - [`RETRO_ENVIRONMENT_SET_HW_RENDER`]
pub const RETRO_ENVIRONMENT_SET_HW_RENDER_CONTEXT_NEGOTIATION_INTERFACE: unsigned =
    43 | RETRO_ENVIRONMENT_EXPERIMENTAL;

/// Notifies the frontend of any quirks associated with serialization.
///
/// Should be set in either `retro_init` or `retro_load_game`, but not both.
///
/// # Parameters
///
/// - inout `uint64_t *data`: Pointer to the core's serialization quirks. The
///   frontend will set the flags of the quirks it supports and clear the flags
///   of those it doesn't. Behavior is undefined if `NULL`.
///
/// # Returns
///
/// `true` if this environment call is supported.
///
/// # See
///
/// - [`retro_serialize`]
/// - [`retro_unserialize`]
/// - [`RETRO_SERIALIZATION_QUIRK`]
pub const RETRO_ENVIRONMENT_SET_SERIALIZATION_QUIRKS: unsigned = 44;

/// The frontend will try to use a "shared" context when setting up a hardware
/// context. Mostly applicable to OpenGL.
///
/// In order for this to have any effect, the core must call
/// `RETRO_ENVIRONMENT_SET_HW_RENDER` at some point if it hasn't already.
///
/// # Parameters
///
/// - `data`: Ignored.
///
/// # Returns
///
/// `true` if the environment call is available and the frontend supports shared
/// hardware contexts.
pub const RETRO_ENVIRONMENT_SET_HW_SHARED_CONTEXT: unsigned = 44 | RETRO_ENVIRONMENT_EXPERIMENTAL;

/// Returns an interface that the core can use to access the file system.
///
/// Should be called as early as possible.
///
/// # Parameters
///
/// - inout `struct retro_vfs_interface_info *data`: Information about the
///   desired VFS interface, as well as the interface itself. Behavior is
///   undefined if `NULL`.
///
/// # Returns
///
/// `true` if this environment call is available and the frontend can provide a
/// VFS interface of the requested version or newer.
///
/// # See
///
/// - [`retro_vfs_interface_info`]
/// - [`file_path`]
/// - [`retro_dirent`]
/// - [`file_stream`]
pub const RETRO_ENVIRONMENT_GET_VFS_INTERFACE: unsigned = 45 | RETRO_ENVIRONMENT_EXPERIMENTAL;

/// Returns an interface that the core can use to set the state of any
/// accessible device LEDs.
///
/// # Parameters
///
/// - output `struct retro_led_interface *data`: Pointer to the LED interface
///   that the frontend will populate. May be `NULL`, in which case the frontend
///   will only return whether this environment callback is available.
///
/// # Returns
///
/// `true` if the environment call is available, even if `data` is `NULL` or no
/// LEDs are accessible.
///
/// # See
///
/// - [`retro_led_interface`]
pub const RETRO_ENVIRONMENT_GET_LED_INTERFACE: unsigned = 46 | RETRO_ENVIRONMENT_EXPERIMENTAL;

/// Returns hints about certain steps that the core may skip for this frame.
///
/// A frontend may not need a core to generate audio or video in certain
/// situations; this environment call sets a bitmask that indicates which steps
/// the core may skip for this frame.
///
/// This can be used to increase performance for some frontend features.
///
/// # Note
///
/// Emulation accuracy should not be compromised; for example, if a core
/// emulates a platform that supports display capture (i.e. looking at its own
/// VRAM), then it should perform its rendering as normal unless it can prove
/// that the emulated game is not using display capture.
///
/// # Parameters
///
/// - output `retro_av_enable_flags *data`: Pointer to the bitmask of steps that
///   the frontend will skip. Other bits are set to zero and are reserved for
///   future use. If `NULL`, the frontend will only return whether this
///   environment callback is available.
///
/// # Returns
///
/// `true` if the environment call is available, regardless of the value output
/// to `data`. If `false`, the core should assume that the frontend will not
/// skip any steps.
///
/// # See
///
/// - [`retro_av_enable_flags`]
pub const RETRO_ENVIRONMENT_GET_AUDIO_VIDEO_ENABLE: unsigned = 47 | RETRO_ENVIRONMENT_EXPERIMENTAL;

/// Gets an interface that the core can use for raw MIDI I/O.
///
/// # Parameters
///
/// - output `struct retro_midi_interface *data`: Pointer to the MIDI interface.
///   May be `NULL`.
///
/// # Returns
///
/// `true` if the environment call is available, even if `data` is `NULL`.
///
/// # See
///
/// - [`retro_midi_interface`]
pub const RETRO_ENVIRONMENT_GET_MIDI_INTERFACE: unsigned = 48 | RETRO_ENVIRONMENT_EXPERIMENTAL;

/// Asks the frontend if it's currently in fast-forward mode.
///
/// # Parameters
///
/// - output `bool *data`: Set to `true` if the frontend is currently
///   fast-forwarding its main loop. Behavior is undefined if `data` is `NULL`.
///
/// # Returns
///
/// `true` if this environment call is available, regardless of the value
/// returned in `data`.
///
/// # See
///
/// - [`RETRO_ENVIRONMENT_SET_FASTFORWARDING_OVERRIDE`]
pub const RETRO_ENVIRONMENT_GET_FASTFORWARDING: unsigned = 49 | RETRO_ENVIRONMENT_EXPERIMENTAL;

/// Returns the refresh rate the frontend is targeting, in Hz. The intended use
/// case is for the core to use the result to select an ideal refresh rate.
///
/// # Parameters
///
/// - output `float *data`: Pointer to the `float` in which the frontend will
///   store its target refresh rate. Behavior is undefined if `data` is `NULL`.
///
/// # Returns
///
/// `true` if this environment call is available, regardless of the value
/// returned in `data`.
pub const RETRO_ENVIRONMENT_GET_TARGET_REFRESH_RATE: unsigned = 50 | RETRO_ENVIRONMENT_EXPERIMENTAL;

/// Returns whether the frontend can return the state of all buttons at once as
/// a bitmask, rather than requiring a series of individual calls to
/// `retro_input_state_t`.
///
/// If this callback returns `true`, you can get the state of all buttons by
/// passing `RETRO_DEVICE_ID_JOYPAD_MASK` as the `id` parameter to
/// `retro_input_state_t`. Bit #N represents the `RETRO_DEVICE_ID_JOYPAD`
/// constant of value N, e.g. `(1 << RETRO_DEVICE_ID_JOYPAD_A)` represents the A
/// button.
///
/// # Parameters
///
/// - `data`: Ignored.
///
/// # Returns
///
/// `true` if the frontend can report the complete digital joypad state as a
/// bitmask.
///
/// # See
///
/// - [`retro_input_state_t`]
/// - [`RETRO_DEVICE_JOYPAD`]
/// - [`RETRO_DEVICE_ID_JOYPAD_MASK`]
pub const RETRO_ENVIRONMENT_GET_INPUT_BITMASKS: unsigned = 51 | RETRO_ENVIRONMENT_EXPERIMENTAL;

/// Returns the version of the core options API supported by the frontend.
///
/// Over the years, libretro has used several interfaces for allowing cores to
/// define customizable options. [`SET_CORE_OPTIONS_V2`] "Version 2 of the
/// interface" is currently preferred due to its extra features, but cores and
/// frontends should strive to support versions
/// [`RETRO_ENVIRONMENT_SET_CORE_OPTIONS`] "1" and
/// [`RETRO_ENVIRONMENT_SET_VARIABLES`] "0" as well. This environment call
/// provides the information that cores need for that purpose.
///
/// If this environment call returns `false`, then the core should assume
/// version 0 of the core options API.
///
/// # Parameters
///
/// - output `unsigned *data`: Pointer to the integer that will store the
///   frontend's supported core options API version. Behavior is undefined if
///   `NULL`.
///
/// # Returns
///
/// `true` if the environment call is available, `false` otherwise.
///
/// # See
///
/// - [`RETRO_ENVIRONMENT_SET_VARIABLES`]
/// - [`RETRO_ENVIRONMENT_SET_CORE_OPTIONS`]
/// - [`RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2`]
pub const RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION: unsigned = 52;

/// Defines a set of core options that can be shown and configured by the
/// frontend, so that the player may customize their gameplay experience to
/// their liking.
///
/// # Deprecated
///
/// This environment call has been superseded by
/// `RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2`, which supports categorizing options
/// into groups. This environment call should only be used to maintain
/// compatibility with older cores and frontends.
///
/// This environment call is intended to replace
/// `RETRO_ENVIRONMENT_SET_VARIABLES`, and should only be called if
/// `RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION` returns an API version of at
/// least 1.
///
/// This should be called the first time as early as possible, ideally in
/// `retro_set_environment` (but `retro_load_game` is acceptable). It may then
/// be called again later to update the core's options and their associated
/// values, as long as the number of options doesn't change from the number
/// given in the first call.
///
/// The core can retrieve option values at any time with
/// `RETRO_ENVIRONMENT_GET_VARIABLE`. If a saved value for a core option doesn't
/// match the option definition's values, the frontend may treat it as incorrect
/// and revert to the default.
///
/// Core options and their values are usually defined in a large static array,
/// but they may be generated at runtime based on the loaded game or system
/// state. Here are some use cases for that:
///
/// - Selecting a particular file from one of the
///   [`RETRO_ENVIRONMENT_GET_ASSET_DIRECTORY`] "frontend's"
///   [`RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY`] "content"
///   [`RETRO_ENVIRONMENT_GET_CORE_ASSETS_DIRECTORY`] "directories", such as a
///   memory card image or figurine data file.
/// - Excluding options that are not relevant to the current game, for cores
///   that define a large number of possible options.
/// - Choosing a default value at runtime for a specific game, such as a BIOS
///   file whose region matches that of the loaded content.
///
/// # Note
///
/// A guiding principle of libretro's API design is that all common interactions
/// (gameplay, menu navigation, etc.) should be possible without a keyboard.
/// This implies that cores should keep the number of options and values as low
/// as possible.
///
/// Example entry:
/// ```c
/// {
///     "foo_option",
///     "Speed hack coprocessor X",
///     "Provides increased performance at the expense of reduced accuracy",
///     {
///         { "false",    NULL },
///         { "true",     NULL },
///         { "unstable", "Turbo (Unstable)" },
///         { NULL, NULL },
///     },
///     "false"
/// }
/// ```
///
/// # Parameters
///
/// - input `const struct retro_core_option_definition *data`: Pointer to one or
///   more core option definitions, terminated by a
///   [`retro_core_option_definition`] whose values are all zero. May be `NULL`,
///   in which case the frontend will remove all existing core options. The
///   frontend must maintain its own copy of this object, including all strings
///   and subobjects.
///
/// # Returns
///
/// `true` if this environment call is available.
///
/// # See
///
/// - [`retro_core_option_definition`]
/// - [`RETRO_ENVIRONMENT_GET_VARIABLE`]
/// - [`RETRO_ENVIRONMENT_SET_CORE_OPTIONS_INTL`]
#[deprecated]
pub const RETRO_ENVIRONMENT_SET_CORE_OPTIONS: unsigned = 53;

/// A variant of [`RETRO_ENVIRONMENT_SET_CORE_OPTIONS`] that supports
/// internationalization.
///
/// # Deprecated
///
/// This environment call has been superseded by
/// [`RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL`], which supports categorizing
/// options into groups (plus translating the groups themselves). Only use this
/// environment call to maintain compatibility with older cores and frontends.
///
/// This should be called instead of `RETRO_ENVIRONMENT_SET_CORE_OPTIONS` if the
/// core provides translations for its options. General use is largely the same,
/// but see [`retro_core_options_intl`] for some important details.
///
/// # Parameters
///
/// - input `const struct retro_core_options_intl *data`: Pointer to a core's
///   option values and their translations.
///
/// # See
///
/// - [`retro_core_options_intl`]
/// - [`RETRO_ENVIRONMENT_SET_CORE_OPTIONS`]
#[deprecated]
pub const RETRO_ENVIRONMENT_SET_CORE_OPTIONS_INTL: unsigned = 54;

/// Notifies the frontend that it should show or hide the named core option.
///
/// Some core options aren't relevant in all scenarios, such as a submenu for
/// hardware rendering flags when the software renderer is configured. This
/// environment call asks the frontend to stop (or start) showing the named core
/// option to the player. This is only a hint, not a requirement; the frontend
/// may ignore this environment call. By default, all core options are visible.
///
/// # Note
///
/// This environment call must *only* affect a core option's visibility, not its
/// functionality or availability. [`RETRO_ENVIRONMENT_GET_VARIABLE`] "Getting
/// an invisible core option" must behave normally.
///
/// # Parameters
///
/// - input `const struct retro_core_option_display *data`: Pointer to a
///   descriptor for the option that the frontend should show or hide. May be
///   `NULL`, in which case the frontend will only return whether this
///   environment callback is available.
///
/// # Returns
///
/// `true` if this environment call is available, even if `data` is `NULL` or
/// the specified option doesn't exist.
///
/// # See
///
/// - [`retro_core_option_display`]
/// - [`RETRO_ENVIRONMENT_SET_CORE_OPTIONS_UPDATE_DISPLAY_CALLBACK`]
pub const RETRO_ENVIRONMENT_SET_CORE_OPTIONS_DISPLAY: unsigned = 55;

/// Returns the frontend's preferred hardware rendering API.
///
/// Cores should use this information to decide which API to use with
/// `RETRO_ENVIRONMENT_SET_HW_RENDER`.
///
/// # Parameters
///
/// - output `retro_hw_context_type *data`: Pointer to the hardware context
///   type. Behavior is undefined if `data` is `NULL`. This value will be set
///   even if the environment call returns `false`, unless the frontend doesn't
///   implement it.
///
/// # Returns
///
/// `true` if the environment call is available and the frontend is able to use
/// a hardware rendering API besides the one returned. If `false` is returned
/// and the core cannot use the preferred rendering API, then it should exit or
/// fall back to software rendering.
///
/// # Note
///
/// The returned value does not indicate which API is currently in use. For
/// example, the frontend may return `RETRO_HW_CONTEXT_OPENGL` while a Direct3D
/// context from a previous session is active; this would signal that the
/// frontend's current preference is for OpenGL, possibly because the user
/// changed their frontend's video driver while a game is running.
///
/// # See
///
/// - [`retro_hw_context_type`]
/// - [`RETRO_ENVIRONMENT_GET_HW_RENDER_INTERFACE`]
/// - [`RETRO_ENVIRONMENT_SET_HW_RENDER`]
pub const RETRO_ENVIRONMENT_GET_PREFERRED_HW_RENDER: unsigned = 56;

/// Returns the minimum version of the disk control interface supported by the
/// frontend.
///
/// If this environment call returns `false` or `data` is 0 or greater, then
/// cores may use disk control callbacks with
/// `RETRO_ENVIRONMENT_SET_DISK_CONTROL_INTERFACE`. If the reported version is 1
/// or greater, then cores should use
/// `RETRO_ENVIRONMENT_SET_DISK_CONTROL_EXT_INTERFACE` instead.
///
/// # Parameters
///
/// - output `unsigned *data`: Pointer to the unsigned integer that the
///   frontend's supported disk control interface version will be stored in.
///   Behavior is undefined if `NULL`.
///
/// # Returns
///
/// `true` if this environment call is available.
///
/// # See
///
/// - [`RETRO_ENVIRONMENT_SET_DISK_CONTROL_EXT_INTERFACE`]
pub const RETRO_ENVIRONMENT_GET_DISK_CONTROL_INTERFACE_VERSION: unsigned = 57;

/// Sets an interface that the frontend can use to insert and remove disks from
/// the emulated console's disk drive.
///
/// Can be used for optical disks, floppy disks, or any other game storage
/// medium that can be swapped at runtime.
///
/// This is intended for multi-disk games that expect the player to manually
/// swap disks at certain points in the game. This version of the disk control
/// interface provides more information about disk images. Should be called in
/// `retro_init`.
///
/// # Parameters
///
/// - input `const struct retro_disk_control_ext_callback *data`: Pointer to the
///   callback functions to use. May be `NULL`, in which case the existing disk
///   callback is deregistered.
///
/// # Returns
///
/// `true` if this environment call is available, even if `data` is `NULL`.
///
/// # See
///
/// - [`retro_disk_control_ext_callback`]
pub const RETRO_ENVIRONMENT_SET_DISK_CONTROL_EXT_INTERFACE: unsigned = 58;

/// Returns the version of the message interface supported by the frontend.
///
/// A version of 0 indicates that the frontend only supports the legacy
/// `RETRO_ENVIRONMENT_SET_MESSAGE` interface. A version of 1 indicates that the
/// frontend supports `RETRO_ENVIRONMENT_SET_MESSAGE_EXT` as well. If this
/// environment call returns `false`, the core should behave as if it had
/// returned 0.
///
/// # Parameters
///
/// - output `unsigned *data`: Pointer to the result returned by the frontend.
///   Behavior is undefined if `NULL`.
///
/// # Returns
///
/// `true` if this environment call is available.
///
/// # See
///
/// - [`RETRO_ENVIRONMENT_SET_MESSAGE_EXT`]
/// - [`RETRO_ENVIRONMENT_SET_MESSAGE`]
pub const RETRO_ENVIRONMENT_GET_MESSAGE_INTERFACE_VERSION: unsigned = 59;

/// Displays a user-facing message for a short time.
///
/// Use this callback to convey important status messages, such as errors or the
/// result of long-running operations. For trivial messages or logging, use
/// `RETRO_ENVIRONMENT_GET_LOG_INTERFACE` or `stderr`.
///
/// This environment call supersedes `RETRO_ENVIRONMENT_SET_MESSAGE`, as it
/// provides many more ways to customize how a message is presented to the
/// player. However, a frontend that supports this environment call must still
/// support `RETRO_ENVIRONMENT_SET_MESSAGE`.
///
/// # Parameters
///
/// - input `const struct retro_message_ext *data`: Pointer to the message to
///   display to the player. Behavior is undefined if `NULL`.
///
/// # Returns
///
/// `true` if this environment call is available.
///
/// # See
///
/// - [`retro_message_ext`]
/// - [`RETRO_ENVIRONMENT_GET_MESSAGE_INTERFACE_VERSION`]
pub const RETRO_ENVIRONMENT_SET_MESSAGE_EXT: unsigned = 60;

/// Returns the number of active input devices currently provided by the
/// frontend.
///
/// This may change between frames, but will remain constant for the duration of
/// each frame.
///
/// If this callback returns `true`, a core need not poll any input device with
/// an index greater than or equal to the returned value.
///
/// If callback returns `false`, the number of active input devices is unknown.
/// In this case, all input devices should be considered active.
///
/// # Parameters
///
/// - output `unsigned *data`: Pointer to the result returned by the frontend.
///   Behavior is undefined if `NULL`.
///
/// # Returns
///
/// `true` if this environment call is available.
pub const RETRO_ENVIRONMENT_GET_INPUT_MAX_USERS: unsigned = 61;

/// Registers a callback that the frontend can use to notify the core of the
/// audio output buffer's occupancy.
///
/// Can be used by a core to attempt frame-skipping to avoid buffer under-runs
/// (i.e. "crackling" sounds).
///
/// # Parameters
///
/// - input `const struct retro_audio_buffer_status_callback *data`: Pointer to
///   the the buffer status callback, or `NULL` to unregister any existing
///   callback.
///
/// # Returns
///
/// `true` if this environment call is available, even if `data` is `NULL`.
///
/// # See
///
/// - [`retro_audio_buffer_status_callback`]
pub const RETRO_ENVIRONMENT_SET_AUDIO_BUFFER_STATUS_CALLBACK: unsigned = 62;

/// Requests a minimum frontend audio latency in milliseconds.
///
/// This is a hint; the frontend may assign a different audio latency to
/// accommodate hardware limits, although it should try to honor requests up to
/// 512ms.
///
/// This callback has no effect if the requested latency is less than the
/// frontend's current audio latency. If value is zero or `data` is `NULL`, the
/// frontend should set its default audio latency.
///
/// May be used by a core to increase audio latency and reduce the risk of
/// buffer under-runs (crackling) when performing 'intensive' operations.
///
/// A core using `RETRO_ENVIRONMENT_SET_AUDIO_BUFFER_STATUS_CALLBACK` to
/// implement audio-buffer-based frame skipping can get good results by setting
/// the audio latency to a high (typically 6x or 8x) integer multiple of the
/// expected frame time.
///
/// This can only be called from within `retro_run`.
///
/// # Warning
///
/// This environment call may require the frontend to reinitialize its audio
/// system. This environment call should be used sparingly. If the driver is
/// reinitialized, [`retro_audio_callback_t`] "all audio callbacks" will be
/// updated to target the newly-initialized driver.
///
/// # Parameters
///
/// - input `const unsigned *data`: Minimum audio latency, in milliseconds.
///
/// # Returns
///
/// `true` if this environment call is available, even if `data` is `NULL`.
///
/// # See
///
/// - [`RETRO_ENVIRONMENT_SET_AUDIO_BUFFER_STATUS_CALLBACK`]
pub const RETRO_ENVIRONMENT_SET_MINIMUM_AUDIO_LATENCY: unsigned = 63;

/// Allows the core to tell the frontend when it should enable fast-forwarding,
/// rather than relying solely on the frontend and user interaction.
///
/// Possible use cases include:
///
/// - Temporarily disabling a core's fastforward support while investigating a
///   related bug.
/// - Disabling fastforward during netplay sessions, or when using an emulated
///   console's network features.
/// - Automatically speeding up the game when in a loading screen that cannot be
///   shortened with high-level emulation.
///
/// # Parameters
///
/// - input `const struct retro_fastforwarding_override *data`: Pointer to the
///   parameters that decide when and how the frontend is allowed to enable
///   fast-forward mode. May be `NULL`, in which case the frontend will return
///   `true` without updating the fastforward state, which can be used to detect
///   support for this environment call.
///
/// # Returns
///
/// `true` if this environment call is available, even if `data` is `NULL`.
///
/// # See
///
/// - [`retro_fastforwarding_override`]
/// - [`RETRO_ENVIRONMENT_GET_FASTFORWARDING`]
pub const RETRO_ENVIRONMENT_SET_FASTFORWARDING_OVERRIDE: unsigned = 64;

/// Allows an implementation to override 'global' content info parameters
/// reported by `retro_get_system_info`.
///
/// Overrides also affect subsystem content info parameters set via
/// `RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO`. This function must be called inside
/// `retro_set_environment`. If callback returns false, content info overrides
/// are unsupported by the frontend, and will be ignored. If callback returns
/// true, extended game info may be retrieved by calling
/// `RETRO_ENVIRONMENT_GET_GAME_INFO_EXT` in `retro_load_game` or
/// `retro_load_game_special`.
///
/// 'data' points to an array of `retro_system_content_info_override` structs
/// terminated by a { NULL, false, false } element. If 'data' is NULL, no
/// changes will be made to the frontend; a core may therefore pass NULL in
/// order to test whether the `RETRO_ENVIRONMENT_SET_CONTENT_INFO_OVERRIDE` and
/// `RETRO_ENVIRONMENT_GET_GAME_INFO_EXT` callbacks are supported by the
/// frontend.
///
/// For struct member descriptions, see the definition of struct
/// `retro_system_content_info_override`.
///
/// Example:
///
/// - struct `retro_system_info`:
///   ```c
///   {
///      "My Core",                      // `library_name`
///      "v1.0",                         // `library_version`
///      "m3u|md|cue|iso|chd|sms|gg|sg", // `valid_extensions`
///      true,                           // `need_fullpath`
///      false                           // `block_extract`
///   }
///   ```
///
/// - Array of struct `retro_system_content_info_override`:
///   ```c
///   {
///      {
///         "md|sms|gg", // extensions
///         false,       // `need_fullpath`
///         true         // `persistent_data`
///      },
///      {
///         "sg",        // extensions
///         false,       // `need_fullpath`
///         false        // `persistent_data`
///      },
///      { NULL, false, false }
///   }
///   ```
///
/// Result:
///
/// - Files of type m3u, cue, iso, chd will not be loaded by the frontend.
///   Frontend will pass a valid path to the core, and core will handle loading
///   internally
/// - Files of type md, sms, gg will be loaded by the frontend. A valid memory
///   buffer will be passed to the core. This memory buffer will remain valid
///   until `retro_deinit` returns
/// - Files of type sg will be loaded by the frontend. A valid memory buffer
///   will be passed to the core. This memory buffer will remain valid until
///   `retro_load_game` (or `retro_load_game_special`) returns
///
/// # Note
///
/// If an extension is listed multiple times in an array of
/// `retro_system_content_info_override` structs, only the first instance will
/// be registered
pub const RETRO_ENVIRONMENT_SET_CONTENT_INFO_OVERRIDE: unsigned = 65;

/// Allows an implementation to fetch extended game information, providing
/// additional content path and memory buffer status details.
///
/// This function may only be called inside `retro_load_game` or
/// `retro_load_game_special`. If callback returns false, extended game
/// information is unsupported by the frontend. In this case, only regular
/// `retro_game_info` will be available. `RETRO_ENVIRONMENT_GET_GAME_INFO_EXT`
/// is guaranteed to return true if
/// `RETRO_ENVIRONMENT_SET_CONTENT_INFO_OVERRIDE` returns true.
///
/// 'data' points to an array of `retro_game_info_ext` structs.
///
/// For struct member descriptions, see the definition of struct
/// `retro_game_info_ext`.
///
/// - If function is called inside `retro_load_game`, the `retro_game_info_ext`
///   array is guaranteed to have a size of 1 - i.e. the returned pointer may be
///   used to access directly the members of the first `retro_game_info_ext`
///   struct, for example:
///   ```c
///   struct retro_game_info_ext *game_info_ext;
///   if (environ_cb(RETRO_ENVIRONMENT_GET_GAME_INFO_EXT, &game_info_ext))
///      printf("Content Directory: %s\n", game_info_ext->dir);
///   ```
/// - If the function is called inside `retro_load_game_special`, the
///   `retro_game_info_ext` array is guaranteed to have a size equal to the
///   `num_info` argument passed to `retro_load_game_special`
pub const RETRO_ENVIRONMENT_GET_GAME_INFO_EXT: unsigned = 66;

/// Defines a set of core options that can be shown and configured by the
/// frontend, so that the player may customize their gameplay experience to
/// their liking.
///
/// # Note
///
/// This environment call is intended to replace
/// `RETRO_ENVIRONMENT_SET_VARIABLES` and `RETRO_ENVIRONMENT_SET_CORE_OPTIONS`,
/// and should only be called if `RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION`
/// returns an API version of at least 2.
///
/// This should be called the first time as early as possible, ideally in
/// `retro_set_environment` (but `retro_load_game` is acceptable). It may then
/// be called again later to update the core's options and their associated
/// values, as long as the number of options doesn't change from the number
/// given in the first call.
///
/// The core can retrieve option values at any time with
/// `RETRO_ENVIRONMENT_GET_VARIABLE`. If a saved value for a core option doesn't
/// match the option definition's values, the frontend may treat it as incorrect
/// and revert to the default.
///
/// Core options and their values are usually defined in a large static array,
/// but they may be generated at runtime based on the loaded game or system
/// state. Here are some use cases for that:
///
/// - Selecting a particular file from one of the
///   [`RETRO_ENVIRONMENT_GET_ASSET_DIRECTORY`] "frontend's"
///   [`RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY`] "content"
///   [`RETRO_ENVIRONMENT_GET_CORE_ASSETS_DIRECTORY`] "directories", such as a
///   memory card image or figurine data file.
/// - Excluding options that are not relevant to the current game, for cores
///   that define a large number of possible options.
/// - Choosing a default value at runtime for a specific game, such as a BIOS
///   file whose region matches that of the loaded content.
///
/// # Note
///
/// A guiding principle of libretro's API design is that all common interactions
/// (gameplay, menu navigation, etc.) should be possible without a keyboard.
/// This implies that cores should keep the number of options and values as low
/// as possible.
///
/// # Parameters
///
/// - input `const struct retro_core_options_v2 *data`: Pointer to a core's
///   options and their associated categories. May be `NULL`, in which case the
///   frontend will remove all existing core options. The frontend must maintain
///   its own copy of this object, including all strings and subobjects.
///
/// # Returns
///
/// `true` if this environment call is available and the frontend supports
/// categories.
///
/// Note that this environment call is guaranteed to successfully register the
/// provided core options, so the return value does not indicate success or
/// failure.
///
/// # See
///
/// - [`retro_core_options_v2`]
/// - [`retro_core_option_v2_category`]
/// - [`retro_core_option_v2_definition`]
/// - [`RETRO_ENVIRONMENT_GET_VARIABLE`]
/// - [`RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL`]
pub const RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2: unsigned = 67;

/// A variant of [`RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2`] that supports
/// internationalization.
///
/// This should be called instead of `RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2` if
/// the core provides translations for its options. General use is largely the
/// same, but see [`retro_core_options_v2_intl`] for some important details.
///
/// # Parameters
///
/// - input `const struct retro_core_options_v2_intl *data`: Pointer to a core's
///   option values and categories, plus a translation for each option and
///   category.
///
/// # See
///
/// - [`retro_core_options_v2_intl`]
/// - [`RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2`]
pub const RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL: unsigned = 68;

/// Registers a callback that the frontend can use to notify the core that at
/// least one core option should be made hidden or visible.
///
/// Allows a frontend to signal that a core must update the visibility of any
/// dynamically hidden core options, and enables the frontend to detect
/// visibility changes. Used by the frontend to update the menu display status
/// of core options without requiring a call of `retro_run`. Must be called in
/// `retro_set_environment`.
///
/// # Parameters
///
/// - input `const struct retro_core_options_update_display_callback *data`: The
///   callback that the frontend should use. May be `NULL`, in which case the
///   frontend will unset any existing callback. Can be used to query visibility
///   support.
///
/// # Returns
///
/// `true` if this environment call is available, even if `data` is `NULL`.
///
/// # See
///
/// - [`retro_core_options_update_display_callback`]
pub const RETRO_ENVIRONMENT_SET_CORE_OPTIONS_UPDATE_DISPLAY_CALLBACK: unsigned = 69;

/// Forcibly sets a core option's value.
///
/// After changing a core option value with this callback, it will be reflected
/// in the frontend and [`RETRO_ENVIRONMENT_GET_VARIABLE_UPDATE`] will return
/// `true`. [`retro_variable::key`] must match a
/// [`RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2`] "previously-set core option", and
/// [`retro_variable::value`] must match one of its defined values.
///
/// Possible use cases include:
///
/// - Allowing the player to set certain core options without entering the
///   frontend's option menu, using an in-core hotkey.
/// - Adjusting invalid combinations of settings.
/// - Migrating settings from older releases of a core.
///
/// # Parameters
///
/// - input `const struct retro_variable *data`: Pointer to a single option that
///   the core is changing. May be `NULL`, in which case the frontend will
///   return `true` to indicate that this environment call is available.
///
/// # Returns
///
/// `true` if this environment call is available and the option named by `key`
/// was successfully set to the given `value`. `false` if the `key` or `value`
/// fields are `NULL`, empty, or don't match a previously set option.
///
/// # See
///
/// - [`RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2`]
/// - [`RETRO_ENVIRONMENT_GET_VARIABLE`]
/// - [`RETRO_ENVIRONMENT_GET_VARIABLE_UPDATE`]
pub const RETRO_ENVIRONMENT_SET_VARIABLE: unsigned = 70;

/// Allows an implementation to get details on the actual rate the frontend is
/// attempting to call `retro_run`.
pub const RETRO_ENVIRONMENT_GET_THROTTLE_STATE: unsigned = 71 | RETRO_ENVIRONMENT_EXPERIMENTAL;

/// Returns information about how the frontend will use savestates.
///
/// # Parameters
///
/// - output `retro_savestate_context *data`: Pointer to the current savestate
///   context. May be `NULL`, in which case the environment call will return
///   `true` to indicate its availability.
///
/// # Returns
///
/// `true` if the environment call is available, even if `data` is `NULL`.
///
/// # See
///
/// - [`retro_savestate_context`]
pub const RETRO_ENVIRONMENT_GET_SAVESTATE_CONTEXT: unsigned = 72 | RETRO_ENVIRONMENT_EXPERIMENTAL;

/// Before calling `SET_HW_RENDER_CONTEXT_NEGOTIATION_INTERFACE`, will query
/// which interface is supported.
///
/// Frontend looks at `retro_hw_render_interface_type` and returns the maximum
/// supported context negotiation interface version. If the
/// `retro_hw_render_interface_type` is not supported or recognized by the
/// frontend, a version of 0 must be returned in `retro_hw_render_interface`'s
/// `interface_version` and `true` is returned by frontend.
///
/// If this environment call returns true with a `interface_version` greater
/// than 0, a core can always use a negotiation interface version larger than
/// what the frontend returns, but only earlier versions of the interface will
/// be used by the frontend.
///
/// A frontend must not reject a negotiation interface version that is larger
/// than what the frontend supports. Instead, the frontend will use the older
/// entry points that it recognizes. If this is incompatible with a particular
/// core's requirements, it can error out early.
///
/// # Note
///
/// Regarding backwards compatibility, this environment call was introduced
/// after Vulkan v1 context negotiation. If this environment call is not
/// supported by frontend, i.e. the environment call returns `false` , only
/// Vulkan v1 context negotiation is supported (if Vulkan HW rendering is
/// supported at all).
///
/// If a core uses Vulkan negotiation interface with version > 1, negotiation
/// may fail unexpectedly. All future updates to the context negotiation
/// interface implies that frontend must support this environment call to query
/// support.
///
/// # Parameters
///
/// - output `struct retro_hw_render_context_negotiation_interface *data`:
///
/// # Returns
///
/// `true` if the environment call is available.
///
/// # See
///
/// - [`SET_HW_RENDER_CONTEXT_NEGOTIATION_INTERFACE`]
/// - [`retro_hw_render_interface_type`]
/// - [`retro_hw_render_context_negotiation_interface`]
pub const RETRO_ENVIRONMENT_GET_HW_RENDER_CONTEXT_NEGOTIATION_INTERFACE_SUPPORT: unsigned =
    73 | RETRO_ENVIRONMENT_EXPERIMENTAL;

/// Asks the frontend whether JIT compilation can be used.
///
/// Primarily used by iOS and tvOS.
///
/// # Parameters
///
/// - output `bool *data`: Set to `true` if the frontend has verified that JIT
///   compilation is possible.
///
/// # Returns
///
/// `true` if the environment call is available.
pub const RETRO_ENVIRONMENT_GET_JIT_CAPABLE: unsigned = 74;

/// Returns an interface that the core can use to receive microphone input.
///
/// # Parameters
///
/// - output `retro_microphone_interface *data`: Pointer to the microphone
///   interface.
///
/// # Returns
///
/// `true` if microphone support is available, even if no microphones are
/// plugged in. `false` if microphone support is disabled unavailable, or if
/// `data` is `NULL`.
///
/// # See
///
/// - [`retro_microphone_interface`]
pub const RETRO_ENVIRONMENT_GET_MICROPHONE_INTERFACE: unsigned =
    75 | RETRO_ENVIRONMENT_EXPERIMENTAL;

// Environment 76 was an obsolete version of
// RETRO_ENVIRONMENT_SET_NETPACKET_INTERFACE. It was not used by any known core
// at the time, and was removed from the API.

/// Returns the device's current power state as reported by the frontend.
///
/// This is useful for emulating the battery level in handheld consoles, or for
/// reducing power consumption when on battery power.
///
/// # Note
///
/// This environment call describes the power state for the entire device, not
/// for individual peripherals like controllers.
///
/// # Parameters
///
/// - output `struct retro_device_power *data`: Indicates whether the frontend
///   can provide this information, even if the parameter is `NULL`. If the
///   frontend does not support this functionality, then the provided argument
///   will remain unchanged.
///
/// # Returns
///
/// `true` if the environment call is available.
///
/// # See
///
/// - [`retro_device_power`]
pub const RETRO_ENVIRONMENT_GET_DEVICE_POWER: unsigned = 77 | RETRO_ENVIRONMENT_EXPERIMENTAL;

/// When set, a core gains control over network packets sent and received during
/// a multiplayer session.
///
/// This can be used to emulate multiplayer games that were originally played on
/// two or more separate consoles or computers connected together.
///
/// The frontend will take care of connecting players together, and the core
/// only needs to send the actual data as needed for the emulation, while
/// handshake and connection management happen in the background.
///
/// When two or more players are connected and this interface has been set, time
/// manipulation features (such as pausing, slow motion, fast forward,
/// rewinding, save state loading, etc.) are disabled to avoid interrupting
/// communication.
///
/// Should be set in either `retro_init` or `retro_load_game`, but not both.
///
/// When not set, a frontend may use state serialization-based multiplayer,
/// where a deterministic core supporting multiple input devices does not need
/// to take any action on its own.
pub const RETRO_ENVIRONMENT_SET_NETPACKET_INTERFACE: unsigned = 78;

/// Returns the "playlist" directory of the frontend.
///
/// This directory can be used to store core generated playlists, in case this
/// internal functionality is available (e.g. internal core game detection
/// engine).
///
/// # Parameters
///
/// - output `const char ** data`: May be `NULL`. If so, no such directory is
///   defined, and it's up to the implementation to find a suitable directory.
///
/// # Returns
///
/// `true` if the environment call is available.
pub const RETRO_ENVIRONMENT_GET_PLAYLIST_DIRECTORY: unsigned = 79;

/// Returns the "file browser" start directory of the frontend.
///
/// This directory can serve as a start directory for the core in case it
/// provides an internal way of loading content.
///
/// # Parameters
///
/// - output `const char ** data`: May be `NULL`. If so, no such directory is
///   defined, and it's up to the implementation to find a suitable directory.
///
/// # Returns
///
/// `true` if the environment call is available.
pub const RETRO_ENVIRONMENT_GET_FILE_BROWSER_START_DIRECTORY: unsigned = 80;
