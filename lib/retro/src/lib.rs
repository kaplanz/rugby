//! # `libretro`
//!
//! Frontend for `rugby` compliant with the `libretro` specification.

#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::wildcard_imports)]
#![allow(non_camel_case_types)]
#![allow(rustdoc::broken_intra_doc_links)]

use std::ffi::CStr;
use std::ops::{Deref, DerefMut};
use std::ptr;

use constcat::concat;
use log::{error, info, warn};
use parking_lot::Mutex;
use rugby::arch::Block;
use rugby::core::dmg::{Button, Cartridge, GameBoy, LCD};
use rugby::emu::part::joypad::State;
use rugby::prelude::*;

pub mod def;
#[rustfmt::skip]
pub mod dev;
pub mod env;
pub mod key;
pub mod loc;
pub mod mem;
pub mod pix;

pub use std::ffi::{
    c_char as char,
    c_double as double,
    c_float as float,
    c_int as int,
    c_uint as unsigned,
    c_void as void,
};

// documentation uses
#[allow(unused_imports)]
use self::def::*;
#[allow(unused_imports)]
use self::dev::*;
#[allow(unused_imports)]
use self::env::*;
#[allow(unused_imports)]
use self::key::*;
#[allow(unused_imports)]
use self::loc::*;
#[allow(unused_imports)]
use self::mem::*;
#[allow(unused_imports)]
use self::pix::*;

/// Shim for accessing the emulation core.
#[derive(Debug, Default)]
struct Emulator(GameBoy);

impl Emulator {
    /// Constructs a new `Emulator`.
    pub fn new() -> Self {
        Self(GameBoy::new())
    }
}

impl Deref for Emulator {
    type Target = GameBoy;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Emulator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

unsafe impl Send for Emulator {}

unsafe impl Sync for Emulator {}

/// Singleton emulation core.
static EMULATOR: Mutex<Option<Emulator>> = Mutex::new(None);

/// The major version of the `libretro` API and ABI.
///
/// Cores may support multiple versions, or they may reject cores with
/// unsupported versions. It is only incremented for incompatible API/ABI
/// changes; this generally implies a function was removed or changed, or that a
/// `struct` had fields removed or changed.
///
/// # Note
///
/// A design goal of `libretro` is to avoid having to increase this value at all
/// costs. This is why there are APIs that are "extended" or "V2".
pub const RETRO_API_VERSION: unsigned = 1;

/// Contains basic information about the core.
///
/// # Warning
///
/// All pointers are owned by the core and must remain valid throughout its
/// lifetime.
///
/// # See
///
/// - [`retro_get_system_info`]
#[derive(Debug)]
#[repr(C)]
pub struct retro_system_info {
    /// Descriptive name of the library.
    ///
    /// # Note
    ///
    /// Should not contain any version numbers, etc.
    pub library_name: *const char,

    /// Descriptive version of the core.
    pub library_version: *const char,

    /// A pipe-delimited string list of file extensions that this core can load.
    ///
    /// Typically used by a frontend for filtering or core selection.
    pub valid_extensions: *const char,

    /// `libretro` cores that need to have direct access to their content files,
    /// including cores which use the path of the content files to determine the
    /// paths of other files, should set `need_fullpath` to true.
    ///
    /// Cores should strive for setting `need_fullpath` to false,
    /// as it allows the frontend to perform patching, etc.
    ///
    /// If `need_fullpath` is true and [`retro_load_game`] is called:
    ///
    /// - [`retro_game_info::path`] is guaranteed to have a valid path
    /// - [`retro_game_info::data`] and [`retro_game_info::size`] are invalid
    ///
    /// If `need_fullpath` is false and [`retro_load_game`] is called:
    ///
    /// - [`retro_game_info::path`] may be `NULL`
    /// - [`retro_game_info::data`] and [`retro_game_info::size`] are guaranteed
    ///   to be valid
    ///
    /// # See
    ///
    /// - [`RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY`]
    /// - [`RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY`]
    pub need_fullpath: bool,

    /// If true, the frontend is not allowed to extract any archives before
    /// loading the real content.
    ///
    /// Necessary for certain `libretro` implementations that load games from
    /// zipped archives.
    pub block_extract: bool,
}

/// Library name string.
//
// SAFETY: `NUL` byte is manually inserted
const NAME: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(concat!(rugby::NAME, '\0').as_bytes()) };

/// Library version string.
//
// SAFETY: `NUL` byte is manually inserted
const VERSION: &CStr = unsafe {
    CStr::from_bytes_with_nul_unchecked(concat!(env!("CARGO_PKG_VERSION"), '\0').as_bytes())
};

/// Core system information.
const SYS_INFO: retro_system_info = retro_system_info {
    library_name: NAME.as_ptr(),
    library_version: VERSION.as_ptr(),
    valid_extensions: ptr::null(),
    need_fullpath: false,
    block_extract: false,
};

/// Parameters describing the size and shape of the video frame.
///
/// # See
///
/// - [`retro_system_av_info`]
/// - [`RETRO_ENVIRONMENT_SET_SYSTEM_AV_INFO`]
/// - [`RETRO_ENVIRONMENT_SET_GEOMETRY`]
/// - [`retro_get_system_av_info`]
#[derive(Debug)]
#[repr(C)]
pub struct retro_game_geometry {
    /// Nominal video width of game, in pixels.
    ///
    /// This will typically be the emulated platform's native video width (or its
    /// smallest, if the original hardware supports multiple resolutions).
    pub base_width: unsigned,

    /// Nominal video height of game, in pixels.
    ///
    /// This will typically be the emulated platform's native video height (or
    /// its smallest, if the original hardware supports multiple resolutions).
    pub base_height: unsigned,

    /// Maximum possible width of the game screen, in pixels.
    ///
    /// This will typically be the emulated platform's maximum video width. For
    /// cores that emulate platforms with multiple screens (such as the Nintendo
    /// DS), this should assume the core's widest possible screen layout (e.g.
    /// side-by-side). For cores that support upscaling the resolution, this
    /// should assume the highest supported scale factor is active.
    pub max_width: unsigned,

    /// Maximum possible height of the game screen, in pixels.
    ///
    /// This will typically be the emulated platform's maximum video height. For
    /// cores that emulate platforms with multiple screens (such as the Nintendo
    /// DS), this should assume the core's tallest possible screen layout (e.g.
    /// vertical). For cores that support upscaling the resolution, this should
    /// assume the highest supported scale factor is active.
    pub max_height: unsigned,

    /// Nominal aspect ratio of game.
    ///
    /// If zero or less, an aspect ratio of `base_width / base_height` is
    /// assumed.
    ///
    /// # Note
    ///
    /// A frontend may ignore this setting.
    pub aspect_ratio: float,
}

/// Parameters describing the timing of the video and audio.
///
/// # See
///
/// - [`retro_system_av_info`]
/// - [`RETRO_ENVIRONMENT_SET_SYSTEM_AV_INFO`]
/// - [`retro_get_system_av_info`]
#[derive(Debug)]
#[repr(C)]
pub struct retro_system_timing {
    /// Video output refresh rate, in frames per second.
    pub fps: double,

    /// The audio output sample rate, in Hz.
    pub sample_rate: double,
}

/// Configures how the core's audio and video should be updated.
///
/// # See
///
/// - [`RETRO_ENVIRONMENT_SET_SYSTEM_AV_INFO`]
/// - [`retro_get_system_av_info`]
#[derive(Debug)]
#[repr(C)]
pub struct retro_system_av_info {
    /// Parameters describing the size and shape of the video frame.
    pub geometry: retro_game_geometry,

    /// Parameters describing the timing of the video and audio.
    pub timing: retro_system_timing,
}

/// Core system audio/video information.
const AV_INFO: retro_system_av_info = retro_system_av_info {
    geometry: retro_game_geometry {
        base_width: LCD.wd as unsigned,
        base_height: LCD.ht as unsigned,
        max_width: LCD.wd as unsigned,
        max_height: LCD.ht as unsigned,
        aspect_ratio: LCD.wd as float / LCD.ht as float,
    },
    timing: retro_system_timing {
        fps: 4_194_304. / 70_224.,
        sample_rate: 44.1e3, // FIXME
    },
};

/// Sets the environment callback.
///
/// # Parameters
///
/// - `cb`: The function which is used when making environment calls.
///
/// # Note
///
/// Guaranteed to be called before [`retro_init`].
///
/// # See
///
/// - [`RETRO_ENVIRONMENT`](mod@env)
#[no_mangle]
pub extern "C" fn retro_set_environment(cb: *const retro_environment_t) {
    let cb = if cb.is_null() {
        return;
    } else {
        // SAFETY: pointer is guaranteed non-null
        #[allow(clippy::crosspointer_transmute)]
        unsafe {
            std::mem::transmute::<*const retro_environment_t, retro_environment_t>(cb)
        }
    };
    let _ = def::ENVIRONMENT
        .set(cb)
        .inspect_err(|_| warn!("race in `retro_set_environment`"));
}

/// Sets the video refresh callback.
///
/// # Parameters
///
/// - `cb`: The function which is used when rendering a frame.
///
/// # Note
///
/// Guaranteed to have been called before the first call to [`retro_run`] is
/// made.
#[no_mangle]
pub extern "C" fn retro_set_video_refresh(cb: *const retro_video_refresh_t) {
    let cb = if cb.is_null() {
        return;
    } else {
        // SAFETY: pointer is guaranteed non-null
        #[allow(clippy::crosspointer_transmute)]
        unsafe {
            std::mem::transmute::<*const retro_video_refresh_t, retro_video_refresh_t>(cb)
        }
    };
    let _ = def::VIDEO_REFRESH
        .set(cb)
        .inspect_err(|_| warn!("race in `retro_set_video_refresh`"));
}

/// Sets the audio sample callback.
///
/// # Parameters
///
/// - `cb`: The function which is used when rendering a single audio frame.
///
/// # Note
///
/// Guaranteed to have been called before the first call to [`retro_run`] is
/// made.
#[no_mangle]
pub extern "C" fn retro_set_audio_sample(cb: *const retro_audio_sample_t) {
    let cb = if cb.is_null() {
        return;
    } else {
        // SAFETY: pointer is guaranteed non-null
        #[allow(clippy::crosspointer_transmute)]
        unsafe {
            std::mem::transmute::<*const retro_audio_sample_t, retro_audio_sample_t>(cb)
        }
    };
    let _ = def::AUDIO_SAMPLE
        .set(cb)
        .inspect_err(|_| warn!("race in `retro_set_audio_sample`"));
}

/// Sets the audio sample batch callback.
///
/// # Parameters
///
/// - `cb`: The function which is used when rendering multiple audio frames in
///   one go.
///
/// # Note
///
/// Guaranteed to have been called before the first call to [`retro_run`] is
/// made.
#[no_mangle]
pub extern "C" fn retro_set_audio_sample_batch(cb: *const retro_audio_sample_batch_t) {
    let cb = if cb.is_null() {
        return;
    } else {
        // SAFETY: pointer is guaranteed non-null
        #[allow(clippy::crosspointer_transmute)]
        unsafe {
            std::mem::transmute::<*const retro_audio_sample_batch_t, retro_audio_sample_batch_t>(cb)
        }
    };
    let _ = def::AUDIO_SAMPLE_BATCH
        .set(cb)
        .inspect_err(|_| warn!("race in `retro_set_audio_sample_batch`"));
}

/// Sets the input poll callback.
///
/// # Parameters
///
/// - `cb`: The function which is used to poll the active input.
///
/// # Note
///
/// Guaranteed to have been called before the first call to [`retro_run`] is
/// made.
#[no_mangle]
pub extern "C" fn retro_set_input_poll(cb: *const retro_input_poll_t) {
    let cb = if cb.is_null() {
        return;
    } else {
        // SAFETY: pointer is guaranteed non-null
        #[allow(clippy::crosspointer_transmute)]
        unsafe {
            std::mem::transmute::<*const retro_input_poll_t, retro_input_poll_t>(cb)
        }
    };
    let _ = def::INPUT_POLL
        .set(cb)
        .inspect_err(|_| warn!("race in `retro_set_input_poll`"));
}

///  Sets the input state callback.
///
/// # Parameters
///
/// - `cb`: The function which is used to query the input state.
///
/// # Note
///
/// Guaranteed to have been called before the first call to [`retro_run`] is
/// made.
#[no_mangle]
pub extern "C" fn retro_set_input_state(cb: *const retro_input_state_t) {
    let cb = if cb.is_null() {
        return;
    } else {
        // SAFETY: pointer is guaranteed non-null
        #[allow(clippy::crosspointer_transmute)]
        unsafe {
            std::mem::transmute::<*const retro_input_state_t, retro_input_state_t>(cb)
        }
    };
    let _ = def::INPUT_STATE
        .set(cb)
        .inspect_err(|_| warn!("race in `retro_set_input_state`"));
}

/// Called by the frontend when initializing a `libretro` core.
///
/// # Warning
///
/// There are many possible "gotchas" with global state in dynamic libraries.
/// Here are some to keep in mind:
///
///  - Do not assume that the core was loaded by the operating system for the
///    first time within this call. It may have been statically linked or
///    retained from a previous session. Consequently, cores must not rely on
///    global variables being initialized to their default values before this
///    function is called; this also goes for object constructors in C++.
///  - Although C++ requires that constructors be called for global variables,
///    it does not require that their destructors be called if stored within a
///    dynamic library's global scope.
///  - If the core is statically linked to the frontend, global variables may be
///    initialized when the frontend itself is initially executed.
///
/// # See
///
/// - [`retro_deinit`]
#[no_mangle]
pub extern "C" fn retro_init() {
    // Construct emulator
    EMULATOR.lock().get_or_insert_with(Emulator::new);

    // Configure environment
    let env = def::ENVIRONMENT
        .get()
        .expect("`retro_set_environment` not initialized");
    env(
        env::RETRO_ENVIRONMENT_SET_PIXEL_FORMAT,
        std::ptr::from_ref::<int>(&(retro_pixel_format::RETRO_PIXEL_FORMAT_XRGB8888 as int))
            .cast::<void>(),
    );
}

/// Called by the frontend when deinitializing a `libretro` core.
///
/// The core must release all of its allocated resources before this function
/// returns.
///
/// # Warning
///
/// There are many possible "gotchas" with global state in dynamic libraries.
/// Here are some to keep in mind:
///
/// - Do not assume that the operating system will unload the core after this
///   function returns, as the core may be linked statically or retained in
///   memory. Cores should use this function to clean up all allocated resources
///   and reset all global variables to their default states.
/// - Do not assume that this core won't be loaded again after this function
///   returns. It may be kept in memory by the frontend for later use, or it may
///   be statically linked. Therefore, all global variables should be reset to
///   their default states within this function.
/// - C++ does not require that destructors be called for variables within a
///   dynamic library's global scope. Therefore, global objects that own
///   dynamically-managed resources (such as `std::string` or `std::vector`)
///   should be kept behind pointers that are explicitly deallocated within this
///   function.
///
/// # See
///
/// - [`retro_init`]
#[no_mangle]
pub extern "C" fn retro_deinit() {
    // Destroy emulator
    EMULATOR.lock().take();
}

/// Retrieves which version of the `libretro` API is being used.
///
/// # Note
///
/// This is used to validate ABI compatibility when the API is revised.
///
/// # Returns
///
/// Must return [`RETRO_API_VERSION`].
///
/// # See
///
/// - [`RETRO_API_VERSION`]
#[no_mangle]
pub extern "C" fn retro_api_version() -> unsigned {
    RETRO_API_VERSION
}

/// Gets statically known system info.
///
/// # Note
///
/// Can be called at any time, even before [`retro_init`].
///
/// # Parameters
///
/// - `info`: A pointer to a [`retro_system_info`] where the info is to be loaded
///   into. This must be statically allocated.
#[no_mangle]
pub extern "C" fn retro_get_system_info(info: *mut retro_system_info) {
    if !info.is_null() {
        // SAFETY: pointer is guaranteed non-null
        unsafe {
            *info = SYS_INFO;
        }
    }
}

/// Gets information about system audio/video timings and geometry.
///
/// # Note
///
/// Can be called only after `retro_load_game` has successfully completed.
///
/// # Note
///
/// The implementation of this function might not initialize every variable if
/// needed. For example, `geom.aspect_ratio` might not be initialized if the
/// core doesn't desire a particular aspect ratio.
///
/// # Parameters
///
/// - `info`: A pointer to a `retro_system_av_info` where the audio/video
///   information should be loaded into.
///
/// - [`retro_system_av_info`]
#[no_mangle]
pub extern "C" fn retro_get_system_av_info(info: *mut retro_system_av_info) {
    if !info.is_null() {
        // SAFETY: pointer is guaranteed non-null
        unsafe {
            *info = AV_INFO;
        }
    }
}

/// Sets device to be used for player 'port'.
///
/// By default, [`RETRO_DEVICE_JOYPAD`] is assumed to be plugged into all
/// available ports.
///
/// # Note
///
/// Setting a particular device type is not a guarantee that `libretro` cores
/// will only poll input based on that particular device type. It is only a hint
/// to the `libretro` core when a core cannot automatically detect the
/// appropriate input device type on its own. It is also relevant when a core
/// can change its behavior depending on device type.
///
/// # Note
///
/// As part of the core's implementation of
/// [`retro_set_controller_port_device`], the core should call
/// [`RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS`] to notify the frontend if the
/// descriptions for any controls have changed as a result of changing the
/// device type.
///
/// # Parameters
///
/// - `port`: Which port to set the device for, usually indicates the player
///   number.
/// - `device`: Which device the given port is using. By default,
///   [`RETRO_DEVICE_JOYPAD`] is assumed for all ports.
///
/// # See
///
/// - [`RETRO_DEVICE_NONE`]
/// - [`RETRO_DEVICE_JOYPAD`]
/// - [`RETRO_DEVICE_MOUSE`]
/// - [`RETRO_DEVICE_KEYBOARD`]
/// - [`RETRO_DEVICE_LIGHTGUN`]
/// - [`RETRO_DEVICE_ANALOG`]
/// - [`RETRO_DEVICE_POINTER`]
/// - [`RETRO_ENVIRONMENT_SET_CONTROLLER_INFO`]
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn retro_set_controller_port_device(port: unsigned, device: unsigned) {
    todo!()
}

/// Resets the currently-loaded game.
///
/// Cores should treat this as a soft reset (i.e. an emulated reset button) if
/// possible, but hard resets are acceptable.
#[no_mangle]
pub extern "C" fn retro_reset() {
    // Acquire emulator instance
    let mut guard = EMULATOR.lock();
    let emu = guard.as_deref_mut().expect("was not initialized");

    // Reset emulator core
    emu.reset();
}

/// Runs the game for one video frame.
///
/// During [`retro_run`], the [`retro_input_poll_t`] callback must be called at
/// least once.
///
/// # Note
///
/// If a frame is not rendered for reasons where a game "dropped" a frame, this
/// still counts as a frame, and [`retro_run`] should explicitly dupe a frame if
/// [`RETRO_ENVIRONMENT_GET_CAN_DUPE`] returns `true`. In this case, the video
/// callback can take a `NULL` argument for data.
///
/// # See
///
/// - [`retro_input_poll_t`]
#[no_mangle]
pub extern "C" fn retro_run() {
    // Acquire emulator instance
    let mut guard = EMULATOR.lock();
    let emu = guard.as_deref_mut().expect("was not initialized");

    // Poll for user input
    let poll = def::INPUT_POLL
        .get()
        .expect("`retro_set_input_poll` not initialized");
    poll();

    // Update input state
    let keys = def::INPUT_STATE
        .get()
        .expect("`retro_set_input_state` not initialized");
    for (key, btn) in [
        (dev::RETRO_DEVICE_ID_JOYPAD_A, Button::A),
        (dev::RETRO_DEVICE_ID_JOYPAD_B, Button::B),
        (dev::RETRO_DEVICE_ID_JOYPAD_SELECT, Button::Select),
        (dev::RETRO_DEVICE_ID_JOYPAD_START, Button::Start),
        (dev::RETRO_DEVICE_ID_JOYPAD_LEFT, Button::Left),
        (dev::RETRO_DEVICE_ID_JOYPAD_RIGHT, Button::Right),
        (dev::RETRO_DEVICE_ID_JOYPAD_UP, Button::Up),
        (dev::RETRO_DEVICE_ID_JOYPAD_DOWN, Button::Down),
    ] {
        // Query external key state
        let state = match keys(0, RETRO_DEVICE_JOYPAD, 0, key) {
            0 => State::Up,
            _ => State::Dn,
        };
        // Update internally button
        emu.inside_mut().joypad().recv(Some((btn, state).into()));
    }

    // Emulate single frame
    let frame = loop {
        // Tick emulator
        emu.cycle();
        // Finish at vertical sync
        if emu.inside().video().vsync() {
            break emu.inside().video().frame();
        }
    };

    // Apply palette to frame
    let frame: Box<[u32]> = frame
        .iter()
        .map(|&pix| rugby::pal::MONO[pix as usize].into())
        .collect();

    // Draw completed frame
    let draw = def::VIDEO_REFRESH
        .get()
        .expect("`retro_set_video_refresh` not initialized");
    draw(
        frame.as_ptr().cast::<void>(),
        unsigned::from(LCD.wd),
        unsigned::from(LCD.ht),
        usize::from(LCD.wd) * std::mem::size_of_val(&frame[0]),
    );
}

/// Returns the amount of data the implementation requires to serialize internal state (save states).
///
/// # Note Between calls to [`retro_load_game`] and [`retro_unload_game`], the
/// returned size is never allowed to be larger than a previous returned value,
/// to ensure that the frontend can allocate a save state buffer once.
///
/// # Return
///
/// The amount of data the implementation requires to serialize the internal
/// state.
///
/// # See
///
/// - [`retro_serialize`]
#[no_mangle]
pub extern "C" fn retro_serialize_size() -> usize {
    todo!()
}

/// Serializes the internal state.
///
/// # Parameters
///
/// - `data`: A pointer to where the serialized data should be saved to.
/// - `size`: The size of the memory.
///
/// # Returns
///
/// If failed, or size is lower than [`retro_serialize_size`], it should return
/// false. On success, it will return true.
///
/// # See
///
/// - [`retro_serialize_size`]
/// - [`retro_unserialize`]
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn retro_serialize(data: *mut void, size: usize) -> bool {
    todo!()
}

/// Unserialize the given state data, and load it into the internal state.
///
/// # Returns
///
/// Returns true if loading the state was successful, false otherwise.
///
/// # See
///
/// - [`retro_serialize`]
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn retro_unserialize(data: *const void, size: usize) -> bool {
    todo!()
}

/// Reset all the active cheats to their default disabled state.
///
/// # See
///
/// - [`retro_cheat_set`]
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn retro_cheat_reset() {
    todo!()
}

/// Enable or disable a cheat.
///
/// # Parameters
///
/// - `index`: The index of the cheat to act upon.
/// - `enabled`: Whether to enable or disable the cheat.
/// - `code`: A string of the code used for the cheat.
///
/// # See
///
/// - [`retro_cheat_reset`]
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn retro_cheat_set(index: unsigned, enabled: bool, code: *const char) {
    todo!()
}

/// Information about a game.
#[derive(Debug)]
#[repr(C)]
pub struct retro_game_info {
    /// Path to game, UTF-8 encoded.
    ///
    /// Sometimes used as a reference for building other paths. May be `NULL` if
    /// game was loaded from stdin or similar, but in this case some cores will
    /// be unable to load `data`. So, it is preferable to fabricate something
    /// here instead of passing `NULL`, which will help more cores to succeed.
    /// [`pub retro_system_info::need_fullpath`] requires that this path is valid.
    pub path: *const char,
    /// Memory buffer of loaded game. Will be `NULL` if
    /// [`retro_system_info::need_fullpath`] was set.
    pub data: *const void,
    /// Size of memory buffer.
    pub size: usize,
    /// String of implementation specific meta-data.
    pub meta: *const char,
}

impl retro_game_info {
    /// Safe wrapper around [`path`](field@Self::path).
    #[must_use]
    pub fn path(&self) -> Option<&CStr> {
        let ptr = self.path;
        if ptr.is_null() {
            None
        } else {
            // SAFETY: pointer is guaranteed non-null
            Some(unsafe { CStr::from_ptr(ptr) })
        }
    }

    /// Safe wrapper around [`data`](field@Self::data).
    #[must_use]
    pub fn data(&self) -> Option<&[u8]> {
        let ptr = self.data.cast::<u8>();
        if ptr.is_null() {
            None
        } else {
            // SAFETY: pointer is guaranteed non-null
            Some(unsafe { std::slice::from_raw_parts(ptr, self.size) })
        }
    }

    /// Safe wrapper around [`meta`](field@Self::meta).
    #[must_use]
    pub fn meta(&self) -> Option<&CStr> {
        let ptr = self.meta;
        if ptr.is_null() {
            None
        } else {
            // SAFETY: pointer is guaranteed non-null
            Some(unsafe { CStr::from_ptr(ptr) })
        }
    }
}

/// Loads a game.
///
/// # Parameters
///
/// - `game`: A pointer to a [`retro_game_info`] detailing information about the
///   game to load. May be `NULL` if the core is loaded without content.
///
/// # Returns
///
/// Will return true when the game was loaded successfully, or false otherwise.
///
/// # See
///
/// - [`retro_game_info`]
/// - [`RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME`]
#[no_mangle]
pub extern "C" fn retro_load_game(game: *const retro_game_info) -> bool {
    // Access game info
    let game: &retro_game_info = if game.is_null() {
        // Report success
        info!("missing game info");
        return true;
    } else {
        // SAFETY: pointer is guaranteed non-null
        unsafe { &*game }
    };

    // Construct game cartridge
    let Some(rom) = game.data() else {
        // Return failure
        return false;
    };
    let cart = match Cartridge::new(rom) {
        Ok(cart) => cart,
        Err(err) => {
            // Report failure
            error!("{err}");
            return false;
        }
    };
    info!("loaded game: {}", cart.title());

    // Acquire emulator instance
    let mut guard = EMULATOR.lock();
    let emu = guard.as_deref_mut().expect("was not initialized");

    // Insert game cartridge
    emu.insert(cart);

    // Report success
    true
}

/// Called when the frontend has loaded one or more "special" content files,
/// typically through subsystems.
///
/// # Note
///
/// Only necessary for cores that support subsystems. Others may return `false`
/// or delegate to [`retro_load_game`].
///
/// # Parameters
///
/// - `game_type`: The type of game to load, as determined by
///   [`retro_subsystem_info`].
/// - `info`: A pointer to an array of [`retro_game_info`] objects providing
///   information about the loaded content.
/// - `num_info`: The number of [`retro_game_info`] objects passed into the info
///   parameter.
///
/// # Returns
///
/// `true` if loading is successful, `false` otherwise. If the core returns
/// `false`, the frontend should abort the core and return to its main menu (if
/// applicable).
///
/// # See
///
/// - [`RETRO_ENVIRONMENT_GET_GAME_INFO_EXT`]
/// - [`RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO`]
/// - [`retro_load_game`]
/// - [`retro_subsystem_info`]
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn retro_load_game_special(
    game_type: unsigned,
    info: *const retro_game_info,
    num_info: usize,
) -> bool {
    todo!()
}

/// Unloads the currently loaded game.
///
/// # Note
///
/// This is called before `retro_deinit`.
///
/// # See
///
/// - [`retro_load_game`]
/// - [`retro_deinit`]
#[no_mangle]
pub extern "C" fn retro_unload_game() {
    // Acquire emulator instance
    let mut guard = EMULATOR.lock();
    let emu = guard.as_deref_mut().expect("was not initialized");

    // Insert game cartridge
    emu.eject()
        .inspect(|cart| info!("ejected game: {}", cart.title()));
}

/// Gets the region of the actively loaded content as either
/// [`RETRO_REGION_NTSC`] or [`RETRO_REGION_PAL`].
///
/// # Note
///
/// This refers to the region of the content's intended television standard, not
/// necessarily the region of the content's origin. For emulated consoles that
/// don't use either standard (e.g. handhelds or post-HD platforms), the core
/// should return [`RETRO_REGION_NTSC`].
///
/// # Returns
///
/// The region of the actively loaded content.
///
/// # See
///
/// - [`RETRO_REGION_NTSC`]
/// - [`RETRO_REGION_PAL`]
#[no_mangle]
pub extern "C" fn retro_get_region() -> unsigned {
    todo!()
}

/// Get a region of memory.
///
/// # Parameters
///
/// - `id`: The ID for the memory block that's desired to retrieve. Can be
///   [`RETRO_MEMORY_SAVE_RAM`], [`RETRO_MEMORY_RTC`],
///   [`RETRO_MEMORY_SYSTEM_RAM`], or [`RETRO_MEMORY_VIDEO_RAM`].
///
/// # Returns
///
/// A pointer to the desired region of memory, or `NULL` when not available.
///
/// # See
///
/// - [`RETRO_MEMORY_SAVE_RAM`]
/// - [`RETRO_MEMORY_RTC`]
/// - [`RETRO_MEMORY_SYSTEM_RAM`]
/// - [`RETRO_MEMORY_VIDEO_RAM`]
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn retro_get_memory_data(id: unsigned) -> *mut void {
    ptr::null_mut() // TODO
}

/// Gets the size of the given region of memory.
///
/// # Parameters
///
/// - `id` The ID for the memory block to check the size of. Can be
///   [`RETRO_MEMORY_SAVE_RAM`], [`RETRO_MEMORY_RTC`],
///   [`RETRO_MEMORY_SYSTEM_RAM`], or [`RETRO_MEMORY_VIDEO_RAM`].
///
/// # Returns
///
/// The size of the region in memory, or 0 when not available.
///
/// # See
///
/// - [`RETRO_MEMORY_SAVE_RAM`]
/// - [`RETRO_MEMORY_RTC`]
/// - [`RETRO_MEMORY_SYSTEM_RAM`]
/// - [`RETRO_MEMORY_VIDEO_RAM`]
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn retro_get_memory_size(id: unsigned) -> usize {
    0 // TODO
}
