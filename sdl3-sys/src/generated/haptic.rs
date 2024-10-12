//! The SDL haptic subsystem manages haptic (force feedback) devices.
//!
//! The basic usage is as follows:
//!
//! - Initialize the subsystem ([`SDL_INIT_HAPTIC`]).
//! - Open a haptic device.
//! - [`SDL_OpenHaptic()`] to open from index.
//! - [`SDL_OpenHapticFromJoystick()`] to open from an existing joystick.
//! - Create an effect ([`SDL_HapticEffect`]).
//! - Upload the effect with [`SDL_CreateHapticEffect()`].
//! - Run the effect with [`SDL_RunHapticEffect()`].
//! - (optional) Free the effect with [`SDL_DestroyHapticEffect()`].
//! - Close the haptic device with [`SDL_CloseHaptic()`].
//!
//! Simple rumble example:
//!
//! ```c
//!    SDL_Haptic *haptic = NULL;
//!
//!    // Open the device
//!    SDL_HapticID *haptics = SDL_GetHaptics(NULL);
//!    if (haptics) {
//!        haptic = SDL_OpenHaptic(haptics[0]);
//!        SDL_free(haptics);
//!    }
//!    if (haptic == NULL)
//!       return;
//!
//!    // Initialize simple rumble
//!    if (!SDL_InitHapticRumble(haptic))
//!       return;
//!
//!    // Play effect at 50% strength for 2 seconds
//!    if (!SDL_PlayHapticRumble(haptic, 0.5, 2000))
//!       return;
//!    SDL_Delay(2000);
//!
//!    // Clean up
//!    SDL_CloseHaptic(haptic);
//! ```
//!
//! Complete example:
//!
//! ```c
//! bool test_haptic(SDL_Joystick *joystick)
//! {
//!    SDL_Haptic *haptic;
//!    SDL_HapticEffect effect;
//!    int effect_id;
//!
//!    // Open the device
//!    haptic = SDL_OpenHapticFromJoystick(joystick);
//!    if (haptic == NULL) return false; // Most likely joystick isn't haptic
//!
//!    // See if it can do sine waves
//!    if ((SDL_GetHapticFeatures(haptic) & SDL_HAPTIC_SINE)==0) {
//!       SDL_CloseHaptic(haptic); // No sine effect
//!       return false;
//!    }
//!
//!    // Create the effect
//!    SDL_memset(&effect, 0, sizeof(SDL_HapticEffect)); // 0 is safe default
//!    effect.type = SDL_HAPTIC_SINE;
//!    effect.periodic.direction.type = SDL_HAPTIC_POLAR; // Polar coordinates
//!    effect.periodic.direction.dir[0] = 18000; // Force comes from south
//!    effect.periodic.period = 1000; // 1000 ms
//!    effect.periodic.magnitude = 20000; // 20000/32767 strength
//!    effect.periodic.length = 5000; // 5 seconds long
//!    effect.periodic.attack_length = 1000; // Takes 1 second to get max strength
//!    effect.periodic.fade_length = 1000; // Takes 1 second to fade away
//!
//!    // Upload the effect
//!    effect_id = SDL_CreateHapticEffect(haptic, &effect);
//!
//!    // Test the effect
//!    SDL_RunHapticEffect(haptic, effect_id, 1);
//!    SDL_Delay(5000); // Wait for the effect to finish
//!
//!    // We destroy the effect, although closing the device also does this
//!    SDL_DestroyHapticEffect(haptic, effect_id);
//!
//!    // Close the device
//!    SDL_CloseHaptic(haptic);
//!
//!    return true; // Success
//! }
//! ```
//!
//! Note that the SDL haptic subsystem is not thread-safe.

use super::stdinc::*;

use super::error::*;

use super::joystick::*;

/// Constant effect supported.
///
/// Constant haptic effect.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_HapticCondition`]<br>
pub const SDL_HAPTIC_CONSTANT: ::core::primitive::u32 = 1_u32;

/// Sine wave effect supported.
///
/// Periodic haptic effect that simulates sine waves.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_HapticPeriodic`]<br>
pub const SDL_HAPTIC_SINE: ::core::primitive::u32 = 2_u32;

/// Square wave effect supported.
///
/// Periodic haptic effect that simulates square waves.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_HapticPeriodic`]<br>
pub const SDL_HAPTIC_SQUARE: ::core::primitive::u32 = 4_u32;

/// Triangle wave effect supported.
///
/// Periodic haptic effect that simulates triangular waves.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_HapticPeriodic`]<br>
pub const SDL_HAPTIC_TRIANGLE: ::core::primitive::u32 = 8_u32;

/// Sawtoothup wave effect supported.
///
/// Periodic haptic effect that simulates saw tooth up waves.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_HapticPeriodic`]<br>
pub const SDL_HAPTIC_SAWTOOTHUP: ::core::primitive::u32 = 16_u32;

/// Sawtoothdown wave effect supported.
///
/// Periodic haptic effect that simulates saw tooth down waves.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_HapticPeriodic`]<br>
pub const SDL_HAPTIC_SAWTOOTHDOWN: ::core::primitive::u32 = 32_u32;

/// Ramp effect supported.
///
/// Ramp haptic effect.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_HapticRamp`]<br>
pub const SDL_HAPTIC_RAMP: ::core::primitive::u32 = 64_u32;

/// Spring effect supported - uses axes position.
///
/// Condition haptic effect that simulates a spring. Effect is based on the
/// axes position.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_HapticCondition`]<br>
pub const SDL_HAPTIC_SPRING: ::core::primitive::u32 = 128_u32;

/// Damper effect supported - uses axes velocity.
///
/// Condition haptic effect that simulates dampening. Effect is based on the
/// axes velocity.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_HapticCondition`]<br>
pub const SDL_HAPTIC_DAMPER: ::core::primitive::u32 = 256_u32;

/// Inertia effect supported - uses axes acceleration.
///
/// Condition haptic effect that simulates inertia. Effect is based on the axes
/// acceleration.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_HapticCondition`]<br>
pub const SDL_HAPTIC_INERTIA: ::core::primitive::u32 = 512_u32;

/// Friction effect supported - uses axes movement.
///
/// Condition haptic effect that simulates friction. Effect is based on the
/// axes movement.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_HapticCondition`]<br>
pub const SDL_HAPTIC_FRICTION: ::core::primitive::u32 = 1024_u32;

/// Left/Right effect supported.
///
/// Haptic effect for direct control over high/low frequency motors.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_HapticLeftRight`]<br>
pub const SDL_HAPTIC_LEFTRIGHT: ::core::primitive::u32 = 2048_u32;

/// Reserved for future use
///
/// This macro is available since SDL 3.0.0.
pub const SDL_HAPTIC_RESERVED1: ::core::primitive::u32 = 4096_u32;

pub const SDL_HAPTIC_RESERVED2: ::core::primitive::u32 = 8192_u32;

pub const SDL_HAPTIC_RESERVED3: ::core::primitive::u32 = 16384_u32;

/// Custom effect is supported.
///
/// User defined custom haptic effect.
///
/// This macro is available since SDL 3.0.0.
pub const SDL_HAPTIC_CUSTOM: ::core::primitive::u32 = 32768_u32;

/// Device can set global gain.
///
/// Device supports setting the global gain.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_SetHapticGain`]<br>
pub const SDL_HAPTIC_GAIN: ::core::primitive::u32 = 65536_u32;

/// Device can set autocenter.
///
/// Device supports setting autocenter.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_SetHapticAutocenter`]<br>
pub const SDL_HAPTIC_AUTOCENTER: ::core::primitive::u32 = 131072_u32;

/// Device can be queried for effect status.
///
/// Device supports querying effect status.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_GetHapticEffectStatus`]<br>
pub const SDL_HAPTIC_STATUS: ::core::primitive::u32 = 262144_u32;

/// Device can be paused.
///
/// Devices supports being paused.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_PauseHaptic`]<br>
/// See also [`SDL_ResumeHaptic`]<br>
pub const SDL_HAPTIC_PAUSE: ::core::primitive::u32 = 524288_u32;

/// Uses polar coordinates for the direction.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_HapticDirection`]<br>
pub const SDL_HAPTIC_POLAR: ::core::primitive::i32 = 0;

/// Uses cartesian coordinates for the direction.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_HapticDirection`]<br>
pub const SDL_HAPTIC_CARTESIAN: ::core::primitive::i32 = 1;

/// Uses spherical coordinates for the direction.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_HapticDirection`]<br>
pub const SDL_HAPTIC_SPHERICAL: ::core::primitive::i32 = 2;

/// Use this value to play an effect on the steering wheel axis.
///
/// This provides better compatibility across platforms and devices as SDL will
/// guess the correct axis.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_HapticDirection`]<br>
pub const SDL_HAPTIC_STEERING_AXIS: ::core::primitive::i32 = 3;

/// Used to play a device an infinite number of times.
///
/// This macro is available since SDL 3.0.0.
///
/// See also [`SDL_RunHapticEffect`]<br>
pub const SDL_HAPTIC_INFINITY: ::core::primitive::u32 = 4294967295_u32;

/// Structure that represents a haptic direction.
///
/// This is the direction where the force comes from, instead of the direction
/// in which the force is exerted.
///
/// Directions can be specified by:
///
/// - [`SDL_HAPTIC_POLAR`] : Specified by polar coordinates.
/// - [`SDL_HAPTIC_CARTESIAN`] : Specified by cartesian coordinates.
/// - [`SDL_HAPTIC_SPHERICAL`] : Specified by spherical coordinates.
///
/// Cardinal directions of the haptic device are relative to the positioning of
/// the device. North is considered to be away from the user.
///
/// The following diagram represents the cardinal directions:
///
/// ```text
///                .--.
///                |__| .-------.
///                |=.| |.-----.|
///                |--| ||     ||
///                |  | |'-----'|
///                |__|~')_____('
///                  [ COMPUTER ]
///
///
///                    North (0,-1)
///                        ^
///                        |
///                        |
///  (-1,0)  West <----[ HAPTIC ]----> East (1,0)
///                        |
///                        |
///                        v
///                     South (0,1)
///
///
///                     [ USER ]
///                       \|||/
///                       (o o)
///                 ---ooO-(_)-Ooo---
/// ```
///
/// If type is [`SDL_HAPTIC_POLAR`], direction is encoded by hundredths of a degree
/// starting north and turning clockwise. [`SDL_HAPTIC_POLAR`] only uses the first
/// `dir` parameter. The cardinal directions would be:
///
/// - North: 0 (0 degrees)
/// - East: 9000 (90 degrees)
/// - South: 18000 (180 degrees)
/// - West: 27000 (270 degrees)
///
/// If type is [`SDL_HAPTIC_CARTESIAN`], direction is encoded by three positions (X
/// axis, Y axis and Z axis (with 3 axes)). [`SDL_HAPTIC_CARTESIAN`] uses the first
/// three `dir` parameters. The cardinal directions would be:
///
/// - North: 0,-1, 0
/// - East: 1, 0, 0
/// - South: 0, 1, 0
/// - West: -1, 0, 0
///
/// The Z axis represents the height of the effect if supported, otherwise it's
/// unused. In cartesian encoding (1, 2) would be the same as (2, 4), you can
/// use any multiple you want, only the direction matters.
///
/// If type is [`SDL_HAPTIC_SPHERICAL`], direction is encoded by two rotations. The
/// first two `dir` parameters are used. The `dir` parameters are as follows
/// (all values are in hundredths of degrees):
///
/// - Degrees from (1, 0) rotated towards (0, 1).
/// - Degrees towards (0, 0, 1) (device needs at least 3 axes).
///
/// Example of force coming from the south with all encodings (force coming
/// from the south means the user will have to pull the stick to counteract):
///
/// ```c
///  SDL_HapticDirection direction;
///
///  // Cartesian directions
///  direction.type = SDL_HAPTIC_CARTESIAN; // Using cartesian direction encoding.
///  direction.dir[0] = 0; // X position
///  direction.dir[1] = 1; // Y position
///  // Assuming the device has 2 axes, we don't need to specify third parameter.
///
///  // Polar directions
///  direction.type = SDL_HAPTIC_POLAR; // We'll be using polar direction encoding.
///  direction.dir[0] = 18000; // Polar only uses first parameter
///
///  // Spherical coordinates
///  direction.type = SDL_HAPTIC_SPHERICAL; // Spherical encoding
///  direction.dir[0] = 9000; // Since we only have two axes we don't need more parameters.
/// ```
///
/// This struct is available since SDL 3.0.0.
///
/// See also [`SDL_HAPTIC_POLAR`]<br>
/// See also [`SDL_HAPTIC_CARTESIAN`]<br>
/// See also [`SDL_HAPTIC_SPHERICAL`]<br>
/// See also [`SDL_HAPTIC_STEERING_AXIS`]<br>
/// See also [`SDL_HapticEffect`]<br>
/// See also [`SDL_GetNumHapticAxes`]<br>
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_HapticDirection {
    /// The type of encoding.
    pub r#type: Uint8,
    /// The encoded direction.
    pub dir: [Sint32; 3],
}

/// A structure containing a template for a Constant effect.
///
/// This struct is exclusively for the [`SDL_HAPTIC_CONSTANT`] effect.
///
/// A constant effect applies a constant force in the specified direction to
/// the joystick.
///
/// This struct is available since SDL 3.0.0.
///
/// See also [`SDL_HAPTIC_CONSTANT`]<br>
/// See also [`SDL_HapticEffect`]<br>
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_HapticConstant {
    /// [`SDL_HAPTIC_CONSTANT`]
    pub r#type: Uint16,
    /// Direction of the effect.
    pub direction: SDL_HapticDirection,
    /// Duration of the effect.
    pub length: Uint32,
    /// Delay before starting the effect.
    pub delay: Uint16,
    /// Button that triggers the effect.
    pub button: Uint16,
    /// How soon it can be triggered again after button.
    pub interval: Uint16,
    /// Strength of the constant effect.
    pub level: Sint16,
    /// Duration of the attack.
    pub attack_length: Uint16,
    /// Level at the start of the attack.
    pub attack_level: Uint16,
    /// Duration of the fade.
    pub fade_length: Uint16,
    /// Level at the end of the fade.
    pub fade_level: Uint16,
}

/// A structure containing a template for a Periodic effect.
///
/// The struct handles the following effects:
///
/// - [`SDL_HAPTIC_SINE`]
/// - [`SDL_HAPTIC_SQUARE`]
/// - [`SDL_HAPTIC_TRIANGLE`]
/// - [`SDL_HAPTIC_SAWTOOTHUP`]
/// - [`SDL_HAPTIC_SAWTOOTHDOWN`]
///
/// A periodic effect consists in a wave-shaped effect that repeats itself over
/// time. The type determines the shape of the wave and the parameters
/// determine the dimensions of the wave.
///
/// Phase is given by hundredth of a degree meaning that giving the phase a
/// value of 9000 will displace it 25% of its period. Here are sample values:
///
/// - 0: No phase displacement.
/// - 9000: Displaced 25% of its period.
/// - 18000: Displaced 50% of its period.
/// - 27000: Displaced 75% of its period.
/// - 36000: Displaced 100% of its period, same as 0, but 0 is preferred.
///
/// Examples:
///
/// ```text
///   SDL_HAPTIC_SINE
///     __      __      __      __
///    /  \    /  \    /  \    /
///   /    \__/    \__/    \__/
///
///   SDL_HAPTIC_SQUARE
///    __    __    __    __    __
///   |  |  |  |  |  |  |  |  |  |
///   |  |__|  |__|  |__|  |__|  |
///
///   SDL_HAPTIC_TRIANGLE
///     /\    /\    /\    /\    /\
///    /  \  /  \  /  \  /  \  /
///   /    \/    \/    \/    \/
///
///   SDL_HAPTIC_SAWTOOTHUP
///     /|  /|  /|  /|  /|  /|  /|
///    / | / | / | / | / | / | / |
///   /  |/  |/  |/  |/  |/  |/  |
///
///   SDL_HAPTIC_SAWTOOTHDOWN
///   \  |\  |\  |\  |\  |\  |\  |
///    \ | \ | \ | \ | \ | \ | \ |
///     \|  \|  \|  \|  \|  \|  \|
/// ```
///
/// This struct is available since SDL 3.0.0.
///
/// See also [`SDL_HAPTIC_SINE`]<br>
/// See also [`SDL_HAPTIC_SQUARE`]<br>
/// See also [`SDL_HAPTIC_TRIANGLE`]<br>
/// See also [`SDL_HAPTIC_SAWTOOTHUP`]<br>
/// See also [`SDL_HAPTIC_SAWTOOTHDOWN`]<br>
/// See also [`SDL_HapticEffect`]<br>
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_HapticPeriodic {
    /// [`SDL_HAPTIC_SINE`], [`SDL_HAPTIC_SQUARE`]
    /// [`SDL_HAPTIC_TRIANGLE`], [`SDL_HAPTIC_SAWTOOTHUP`] or
    /// [`SDL_HAPTIC_SAWTOOTHDOWN`]
    pub r#type: Uint16,
    /// Direction of the effect.
    pub direction: SDL_HapticDirection,
    /// Duration of the effect.
    pub length: Uint32,
    /// Delay before starting the effect.
    pub delay: Uint16,
    /// Button that triggers the effect.
    pub button: Uint16,
    /// How soon it can be triggered again after button.
    pub interval: Uint16,
    /// Period of the wave.
    pub period: Uint16,
    /// Peak value; if negative, equivalent to 180 degrees extra phase shift.
    pub magnitude: Sint16,
    /// Mean value of the wave.
    pub offset: Sint16,
    /// Positive phase shift given by hundredth of a degree.
    pub phase: Uint16,
    /// Duration of the attack.
    pub attack_length: Uint16,
    /// Level at the start of the attack.
    pub attack_level: Uint16,
    /// Duration of the fade.
    pub fade_length: Uint16,
    /// Level at the end of the fade.
    pub fade_level: Uint16,
}

/// A structure containing a template for a Condition effect.
///
/// The struct handles the following effects:
///
/// - SDL_HAPTIC_SPRING: Effect based on axes position.
/// - SDL_HAPTIC_DAMPER: Effect based on axes velocity.
/// - SDL_HAPTIC_INERTIA: Effect based on axes acceleration.
/// - SDL_HAPTIC_FRICTION: Effect based on axes movement.
///
/// Direction is handled by condition internals instead of a direction member.
/// The condition effect specific members have three parameters. The first
/// refers to the X axis, the second refers to the Y axis and the third refers
/// to the Z axis. The right terms refer to the positive side of the axis and
/// the left terms refer to the negative side of the axis. Please refer to the
/// [`SDL_HapticDirection`] diagram for which side is positive and which is
/// negative.
///
/// This struct is available since SDL 3.0.0.
///
/// See also [`SDL_HapticDirection`]<br>
/// See also [`SDL_HAPTIC_SPRING`]<br>
/// See also [`SDL_HAPTIC_DAMPER`]<br>
/// See also [`SDL_HAPTIC_INERTIA`]<br>
/// See also [`SDL_HAPTIC_FRICTION`]<br>
/// See also [`SDL_HapticEffect`]<br>
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_HapticCondition {
    /// [`SDL_HAPTIC_SPRING`], [`SDL_HAPTIC_DAMPER`],
    /// [`SDL_HAPTIC_INERTIA`] or [`SDL_HAPTIC_FRICTION`]
    pub r#type: Uint16,
    /// Direction of the effect - Not used ATM.
    pub direction: SDL_HapticDirection,
    /// Duration of the effect.
    pub length: Uint32,
    /// Delay before starting the effect.
    pub delay: Uint16,
    /// Button that triggers the effect.
    pub button: Uint16,
    /// How soon it can be triggered again after button.
    pub interval: Uint16,
    /// Level when joystick is to the positive side; max 0xFFFF.
    pub right_sat: [Uint16; 3],
    /// Level when joystick is to the negative side; max 0xFFFF.
    pub left_sat: [Uint16; 3],
    /// How fast to increase the force towards the positive side.
    pub right_coeff: [Sint16; 3],
    /// How fast to increase the force towards the negative side.
    pub left_coeff: [Sint16; 3],
    /// Size of the dead zone; max 0xFFFF: whole axis-range when 0-centered.
    pub deadband: [Uint16; 3],
    /// Position of the dead zone.
    pub center: [Sint16; 3],
}

/// A structure containing a template for a Ramp effect.
///
/// This struct is exclusively for the [`SDL_HAPTIC_RAMP`] effect.
///
/// The ramp effect starts at start strength and ends at end strength. It
/// augments in linear fashion. If you use attack and fade with a ramp the
/// effects get added to the ramp effect making the effect become quadratic
/// instead of linear.
///
/// This struct is available since SDL 3.0.0.
///
/// See also [`SDL_HAPTIC_RAMP`]<br>
/// See also [`SDL_HapticEffect`]<br>
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_HapticRamp {
    /// [`SDL_HAPTIC_RAMP`]
    pub r#type: Uint16,
    /// Direction of the effect.
    pub direction: SDL_HapticDirection,
    /// Duration of the effect.
    pub length: Uint32,
    /// Delay before starting the effect.
    pub delay: Uint16,
    /// Button that triggers the effect.
    pub button: Uint16,
    /// How soon it can be triggered again after button.
    pub interval: Uint16,
    /// Beginning strength level.
    pub start: Sint16,
    /// Ending strength level.
    pub end: Sint16,
    /// Duration of the attack.
    pub attack_length: Uint16,
    /// Level at the start of the attack.
    pub attack_level: Uint16,
    /// Duration of the fade.
    pub fade_length: Uint16,
    /// Level at the end of the fade.
    pub fade_level: Uint16,
}

/// A structure containing a template for a Left/Right effect.
///
/// This struct is exclusively for the [`SDL_HAPTIC_LEFTRIGHT`] effect.
///
/// The Left/Right effect is used to explicitly control the large and small
/// motors, commonly found in modern game controllers. The small (right) motor
/// is high frequency, and the large (left) motor is low frequency.
///
/// This struct is available since SDL 3.0.0.
///
/// See also [`SDL_HAPTIC_LEFTRIGHT`]<br>
/// See also [`SDL_HapticEffect`]<br>
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_HapticLeftRight {
    /// [`SDL_HAPTIC_LEFTRIGHT`]
    pub r#type: Uint16,
    /// Duration of the effect in milliseconds.
    pub length: Uint32,
    /// Control of the large controller motor.
    pub large_magnitude: Uint16,
    /// Control of the small controller motor.
    pub small_magnitude: Uint16,
}

/// A structure containing a template for the [`SDL_HAPTIC_CUSTOM`] effect.
///
/// This struct is exclusively for the [`SDL_HAPTIC_CUSTOM`] effect.
///
/// A custom force feedback effect is much like a periodic effect, where the
/// application can define its exact shape. You will have to allocate the data
/// yourself. Data should consist of channels * samples Uint16 samples.
///
/// If channels is one, the effect is rotated using the defined direction.
/// Otherwise it uses the samples in data for the different axes.
///
/// This struct is available since SDL 3.0.0.
///
/// See also [`SDL_HAPTIC_CUSTOM`]<br>
/// See also [`SDL_HapticEffect`]<br>
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_HapticCustom {
    /// [`SDL_HAPTIC_CUSTOM`]
    pub r#type: Uint16,
    /// Direction of the effect.
    pub direction: SDL_HapticDirection,
    /// Duration of the effect.
    pub length: Uint32,
    /// Delay before starting the effect.
    pub delay: Uint16,
    /// Button that triggers the effect.
    pub button: Uint16,
    /// How soon it can be triggered again after button.
    pub interval: Uint16,
    /// Axes to use, minimum of one.
    pub channels: Uint8,
    /// Sample periods.
    pub period: Uint16,
    /// Amount of samples.
    pub samples: Uint16,
    /// Should contain channels*samples items.
    pub data: *mut Uint16,
    /// Duration of the attack.
    pub attack_length: Uint16,
    /// Level at the start of the attack.
    pub attack_level: Uint16,
    /// Duration of the fade.
    pub fade_length: Uint16,
    /// Level at the end of the fade.
    pub fade_level: Uint16,
}

/// The generic template for any haptic effect.
///
/// All values max at 32767 (0x7FFF). Signed values also can be negative. Time
/// values unless specified otherwise are in milliseconds.
///
/// You can also pass [`SDL_HAPTIC_INFINITY`] to length instead of a 0-32767 value.
/// Neither delay, interval, attack_length nor fade_length support
/// [`SDL_HAPTIC_INFINITY`]. Fade will also not be used since effect never ends.
///
/// Additionally, the [`SDL_HAPTIC_RAMP`] effect does not support a duration of
/// [`SDL_HAPTIC_INFINITY`].
///
/// Button triggers may not be supported on all devices, it is advised to not
/// use them if possible. Buttons start at index 1 instead of index 0 like the
/// joystick.
///
/// If both attack_length and fade_level are 0, the envelope is not used,
/// otherwise both values are used.
///
/// Common parts:
///
/// ```c
///  // Replay - All effects have this
///  Uint32 length;        // Duration of effect (ms).
///  Uint16 delay;         // Delay before starting effect.
///
///  // Trigger - All effects have this
///  Uint16 button;        // Button that triggers effect.
///  Uint16 interval;      // How soon before effect can be triggered again.
///
///  // Envelope - All effects except condition effects have this
///  Uint16 attack_length; // Duration of the attack (ms).
///  Uint16 attack_level;  // Level at the start of the attack.
///  Uint16 fade_length;   // Duration of the fade out (ms).
///  Uint16 fade_level;    // Level at the end of the fade.
/// ```
///
/// Here we have an example of a constant effect evolution in time:
///
/// ```text
///  Strength
///  ^
///  |
///  |    effect level -->  _________________
///  |                     /                 \
///  |                    /                   \
///  |                   /                     \
///  |                  /                       \
///  | attack_level --> |                        \
///  |                  |                        |  <---  fade_level
///  |
///  +--------------------------------------------------> Time
///                     [--]                 [---]
///                     attack_length        fade_length
///
///  [------------------][-----------------------]
///  delay               length
/// ```
///
/// Note either the attack_level or the fade_level may be above the actual
/// effect level.
///
/// This struct is available since SDL 3.0.0.
///
/// See also [`SDL_HapticConstant`]<br>
/// See also [`SDL_HapticPeriodic`]<br>
/// See also [`SDL_HapticCondition`]<br>
/// See also [`SDL_HapticRamp`]<br>
/// See also [`SDL_HapticLeftRight`]<br>
/// See also [`SDL_HapticCustom`]<br>
#[repr(C)]
#[derive(Clone, Copy)]
pub union SDL_HapticEffect {
    /// Effect type.
    pub r#type: Uint16,
    /// Constant effect.
    pub constant: SDL_HapticConstant,
    /// Periodic effect.
    pub periodic: SDL_HapticPeriodic,
    /// Condition effect.
    pub condition: SDL_HapticCondition,
    /// Ramp effect.
    pub ramp: SDL_HapticRamp,
    /// Left/Right effect.
    pub leftright: SDL_HapticLeftRight,
    /// Custom effect.
    pub custom: SDL_HapticCustom,
}

/// This is a unique ID for a haptic device for the time it is connected to the
/// system, and is never reused for the lifetime of the application.
///
/// If the haptic device is disconnected and reconnected, it will get a new ID.
///
/// The value 0 is an invalid ID.
///
/// This datatype is available since SDL 3.0.0.
pub type SDL_HapticID = Uint32;

extern "C" {
    /// Get a list of currently connected haptic devices.
    ///
    /// - `count`: a pointer filled in with the number of haptic devices
    ///   returned, may be NULL.
    /// - Returns a 0 terminated array of haptic device instance IDs or NULL on
    ///   failure; call [`SDL_GetError()`] for more information. This should be
    ///   freed with [`SDL_free()`] when it is no longer needed.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_OpenHaptic`]<br>
    pub fn SDL_GetHaptics(count: *mut ::core::ffi::c_int) -> *mut SDL_HapticID;
}

extern "C" {
    /// Get the implementation dependent name of a haptic device.
    ///
    /// This can be called before any haptic devices are opened.
    ///
    /// - `instance_id`: the haptic device instance ID.
    /// - Returns the name of the selected haptic device. If no name can be found,
    ///   this function returns NULL; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_GetHapticName`]<br>
    /// See also [`SDL_OpenHaptic`]<br>
    pub fn SDL_GetHapticNameForID(instance_id: SDL_HapticID) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Open a haptic device for use.
    ///
    /// The index passed as an argument refers to the N'th haptic device on this
    /// system.
    ///
    /// When opening a haptic device, its gain will be set to maximum and
    /// autocenter will be disabled. To modify these values use [`SDL_SetHapticGain()`]
    /// and [`SDL_SetHapticAutocenter()`].
    ///
    /// - `instance_id`: the haptic device instance ID.
    /// - Returns the device identifier or NULL on failure; call [`SDL_GetError()`] for
    ///   more information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_CloseHaptic`]<br>
    /// See also [`SDL_GetHaptics`]<br>
    /// See also [`SDL_OpenHapticFromJoystick`]<br>
    /// See also [`SDL_OpenHapticFromMouse`]<br>
    /// See also [`SDL_SetHapticAutocenter`]<br>
    /// See also [`SDL_SetHapticGain`]<br>
    pub fn SDL_OpenHaptic(instance_id: SDL_HapticID) -> *mut SDL_Haptic;
}

extern "C" {
    /// Get the [`SDL_Haptic`] associated with an instance ID, if it has been opened.
    ///
    /// - `instance_id`: the instance ID to get the [`SDL_Haptic`] for.
    /// - Returns an [`SDL_Haptic`] on success or NULL on failure or if it hasn't been
    ///   opened yet; call [`SDL_GetError()`] for more information.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_GetHapticFromID(instance_id: SDL_HapticID) -> *mut SDL_Haptic;
}

extern "C" {
    /// Get the instance ID of an opened haptic device.
    ///
    /// - `haptic`: the [`SDL_Haptic`] device to query.
    /// - Returns the instance ID of the specified haptic device on success or 0 on
    ///   failure; call [`SDL_GetError()`] for more information.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_GetHapticID(haptic: *mut SDL_Haptic) -> SDL_HapticID;
}

extern "C" {
    /// Get the implementation dependent name of a haptic device.
    ///
    /// - `haptic`: the [`SDL_Haptic`] obtained from [`SDL_OpenJoystick()`].
    /// - Returns the name of the selected haptic device. If no name can be found,
    ///   this function returns NULL; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_GetHapticNameForID`]<br>
    pub fn SDL_GetHapticName(haptic: *mut SDL_Haptic) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Query whether or not the current mouse has haptic capabilities.
    ///
    /// - Returns true if the mouse is haptic or false if it isn't.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_OpenHapticFromMouse`]<br>
    pub fn SDL_IsMouseHaptic() -> ::core::primitive::bool;
}

extern "C" {
    /// Try to open a haptic device from the current mouse.
    ///
    /// - Returns the haptic device identifier or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_CloseHaptic`]<br>
    /// See also [`SDL_IsMouseHaptic`]<br>
    pub fn SDL_OpenHapticFromMouse() -> *mut SDL_Haptic;
}

extern "C" {
    /// Query if a joystick has haptic features.
    ///
    /// - `joystick`: the [`SDL_Joystick`] to test for haptic capabilities.
    /// - Returns true if the joystick is haptic or false if it isn't.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_OpenHapticFromJoystick`]<br>
    pub fn SDL_IsJoystickHaptic(joystick: *mut SDL_Joystick) -> ::core::primitive::bool;
}

extern "C" {
    /// Open a haptic device for use from a joystick device.
    ///
    /// You must still close the haptic device separately. It will not be closed
    /// with the joystick.
    ///
    /// When opened from a joystick you should first close the haptic device before
    /// closing the joystick device. If not, on some implementations the haptic
    /// device will also get unallocated and you'll be unable to use force feedback
    /// on that device.
    ///
    /// - `joystick`: the [`SDL_Joystick`] to create a haptic device from.
    /// - Returns a valid haptic device identifier on success or NULL on failure;
    ///   call [`SDL_GetError()`] for more information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_CloseHaptic`]<br>
    /// See also [`SDL_IsJoystickHaptic`]<br>
    pub fn SDL_OpenHapticFromJoystick(joystick: *mut SDL_Joystick) -> *mut SDL_Haptic;
}

extern "C" {
    /// Close a haptic device previously opened with [`SDL_OpenHaptic()`].
    ///
    /// - `haptic`: the [`SDL_Haptic`] device to close.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_OpenHaptic`]<br>
    pub fn SDL_CloseHaptic(haptic: *mut SDL_Haptic);
}

extern "C" {
    /// Get the number of effects a haptic device can store.
    ///
    /// On some platforms this isn't fully supported, and therefore is an
    /// approximation. Always check to see if your created effect was actually
    /// created and do not rely solely on [`SDL_GetMaxHapticEffects()`].
    ///
    /// - `haptic`: the [`SDL_Haptic`] device to query.
    /// - Returns the number of effects the haptic device can store or a negative
    ///   error code on failure; call [`SDL_GetError()`] for more information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_GetMaxHapticEffectsPlaying`]<br>
    /// See also [`SDL_GetHapticFeatures`]<br>
    pub fn SDL_GetMaxHapticEffects(haptic: *mut SDL_Haptic) -> ::core::ffi::c_int;
}

extern "C" {
    /// Get the number of effects a haptic device can play at the same time.
    ///
    /// This is not supported on all platforms, but will always return a value.
    ///
    /// - `haptic`: the [`SDL_Haptic`] device to query maximum playing effects.
    /// - Returns the number of effects the haptic device can play at the same time
    ///   or -1 on failure; call [`SDL_GetError()`] for more information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_GetMaxHapticEffects`]<br>
    /// See also [`SDL_GetHapticFeatures`]<br>
    pub fn SDL_GetMaxHapticEffectsPlaying(haptic: *mut SDL_Haptic) -> ::core::ffi::c_int;
}

extern "C" {
    /// Get the haptic device's supported features in bitwise manner.
    ///
    /// - `haptic`: the [`SDL_Haptic`] device to query.
    /// - Returns a list of supported haptic features in bitwise manner (OR'd), or 0
    ///   on failure; call [`SDL_GetError()`] for more information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_HapticEffectSupported`]<br>
    /// See also [`SDL_GetMaxHapticEffects`]<br>
    pub fn SDL_GetHapticFeatures(haptic: *mut SDL_Haptic) -> Uint32;
}

extern "C" {
    /// Get the number of haptic axes the device has.
    ///
    /// The number of haptic axes might be useful if working with the
    /// [`SDL_HapticDirection`] effect.
    ///
    /// - `haptic`: the [`SDL_Haptic`] device to query.
    /// - Returns the number of axes on success or -1 on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// This function is available since SDL 3.0.0.
    pub fn SDL_GetNumHapticAxes(haptic: *mut SDL_Haptic) -> ::core::ffi::c_int;
}

extern "C" {
    /// Check to see if an effect is supported by a haptic device.
    ///
    /// - `haptic`: the [`SDL_Haptic`] device to query.
    /// - `effect`: the desired effect to query.
    /// - Returns true if the effect is supported or false if it isn't.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_CreateHapticEffect`]<br>
    /// See also [`SDL_GetHapticFeatures`]<br>
    pub fn SDL_HapticEffectSupported(
        haptic: *mut SDL_Haptic,
        effect: *const SDL_HapticEffect,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Create a new haptic effect on a specified device.
    ///
    /// - `haptic`: an [`SDL_Haptic`] device to create the effect on.
    /// - `effect`: an [`SDL_HapticEffect`] structure containing the properties of
    ///   the effect to create.
    /// - Returns the ID of the effect on success or -1 on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_DestroyHapticEffect`]<br>
    /// See also [`SDL_RunHapticEffect`]<br>
    /// See also [`SDL_UpdateHapticEffect`]<br>
    pub fn SDL_CreateHapticEffect(
        haptic: *mut SDL_Haptic,
        effect: *const SDL_HapticEffect,
    ) -> ::core::ffi::c_int;
}

extern "C" {
    /// Update the properties of an effect.
    ///
    /// Can be used dynamically, although behavior when dynamically changing
    /// direction may be strange. Specifically the effect may re-upload itself and
    /// start playing from the start. You also cannot change the type either when
    /// running [`SDL_UpdateHapticEffect()`].
    ///
    /// - `haptic`: the [`SDL_Haptic`] device that has the effect.
    /// - `effect`: the identifier of the effect to update.
    /// - `data`: an [`SDL_HapticEffect`] structure containing the new effect
    ///   properties to use.
    /// - Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_CreateHapticEffect`]<br>
    /// See also [`SDL_RunHapticEffect`]<br>
    pub fn SDL_UpdateHapticEffect(
        haptic: *mut SDL_Haptic,
        effect: ::core::ffi::c_int,
        data: *const SDL_HapticEffect,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Run the haptic effect on its associated haptic device.
    ///
    /// To repeat the effect over and over indefinitely, set `iterations` to
    /// `SDL_HAPTIC_INFINITY`. (Repeats the envelope - attack and fade.) To make
    /// one instance of the effect last indefinitely (so the effect does not fade),
    /// set the effect's `length` in its structure/union to `SDL_HAPTIC_INFINITY`
    /// instead.
    ///
    /// - `haptic`: the [`SDL_Haptic`] device to run the effect on.
    /// - `effect`: the ID of the haptic effect to run.
    /// - `iterations`: the number of iterations to run the effect; use
    ///   `SDL_HAPTIC_INFINITY` to repeat forever.
    /// - Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_GetHapticEffectStatus`]<br>
    /// See also [`SDL_StopHapticEffect`]<br>
    /// See also [`SDL_StopHapticEffects`]<br>
    pub fn SDL_RunHapticEffect(
        haptic: *mut SDL_Haptic,
        effect: ::core::ffi::c_int,
        iterations: Uint32,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Stop the haptic effect on its associated haptic device.
    ///
    /// - `haptic`: the [`SDL_Haptic`] device to stop the effect on.
    /// - `effect`: the ID of the haptic effect to stop.
    /// - Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_RunHapticEffect`]<br>
    /// See also [`SDL_StopHapticEffects`]<br>
    pub fn SDL_StopHapticEffect(
        haptic: *mut SDL_Haptic,
        effect: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Destroy a haptic effect on the device.
    ///
    /// This will stop the effect if it's running. Effects are automatically
    /// destroyed when the device is closed.
    ///
    /// - `haptic`: the [`SDL_Haptic`] device to destroy the effect on.
    /// - `effect`: the ID of the haptic effect to destroy.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_CreateHapticEffect`]<br>
    pub fn SDL_DestroyHapticEffect(haptic: *mut SDL_Haptic, effect: ::core::ffi::c_int);
}

extern "C" {
    /// Get the status of the current effect on the specified haptic device.
    ///
    /// Device must support the [`SDL_HAPTIC_STATUS`] feature.
    ///
    /// - `haptic`: the [`SDL_Haptic`] device to query for the effect status on.
    /// - `effect`: the ID of the haptic effect to query its status.
    /// - Returns true if it is playing, false if it isn't playing or haptic status
    ///   isn't supported.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_GetHapticFeatures`]<br>
    pub fn SDL_GetHapticEffectStatus(
        haptic: *mut SDL_Haptic,
        effect: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Set the global gain of the specified haptic device.
    ///
    /// Device must support the [`SDL_HAPTIC_GAIN`] feature.
    ///
    /// The user may specify the maximum gain by setting the environment variable
    /// `SDL_HAPTIC_GAIN_MAX` which should be between 0 and 100. All calls to
    /// [`SDL_SetHapticGain()`] will scale linearly using `SDL_HAPTIC_GAIN_MAX` as the
    /// maximum.
    ///
    /// - `haptic`: the [`SDL_Haptic`] device to set the gain on.
    /// - `gain`: value to set the gain to, should be between 0 and 100 (0 -
    ///   100).
    /// - Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_GetHapticFeatures`]<br>
    pub fn SDL_SetHapticGain(
        haptic: *mut SDL_Haptic,
        gain: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Set the global autocenter of the device.
    ///
    /// Autocenter should be between 0 and 100. Setting it to 0 will disable
    /// autocentering.
    ///
    /// Device must support the [`SDL_HAPTIC_AUTOCENTER`] feature.
    ///
    /// - `haptic`: the [`SDL_Haptic`] device to set autocentering on.
    /// - `autocenter`: value to set autocenter to (0-100).
    /// - Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_GetHapticFeatures`]<br>
    pub fn SDL_SetHapticAutocenter(
        haptic: *mut SDL_Haptic,
        autocenter: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Pause a haptic device.
    ///
    /// Device must support the `SDL_HAPTIC_PAUSE` feature. Call [`SDL_ResumeHaptic()`]
    /// to resume playback.
    ///
    /// Do not modify the effects nor add new ones while the device is paused. That
    /// can cause all sorts of weird errors.
    ///
    /// - `haptic`: the [`SDL_Haptic`] device to pause.
    /// - Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_ResumeHaptic`]<br>
    pub fn SDL_PauseHaptic(haptic: *mut SDL_Haptic) -> ::core::primitive::bool;
}

extern "C" {
    /// Resume a haptic device.
    ///
    /// Call to unpause after [`SDL_PauseHaptic()`].
    ///
    /// - `haptic`: the [`SDL_Haptic`] device to unpause.
    /// - Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_PauseHaptic`]<br>
    pub fn SDL_ResumeHaptic(haptic: *mut SDL_Haptic) -> ::core::primitive::bool;
}

extern "C" {
    /// Stop all the currently playing effects on a haptic device.
    ///
    /// - `haptic`: the [`SDL_Haptic`] device to stop.
    /// - Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_RunHapticEffect`]<br>
    /// See also [`SDL_StopHapticEffects`]<br>
    pub fn SDL_StopHapticEffects(haptic: *mut SDL_Haptic) -> ::core::primitive::bool;
}

extern "C" {
    /// Check whether rumble is supported on a haptic device.
    ///
    /// - `haptic`: haptic device to check for rumble support.
    /// - Returns true if the effect is supported or false if it isn't.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_InitHapticRumble`]<br>
    pub fn SDL_HapticRumbleSupported(haptic: *mut SDL_Haptic) -> ::core::primitive::bool;
}

extern "C" {
    /// Initialize a haptic device for simple rumble playback.
    ///
    /// - `haptic`: the haptic device to initialize for simple rumble playback.
    /// - Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_PlayHapticRumble`]<br>
    /// See also [`SDL_StopHapticRumble`]<br>
    /// See also [`SDL_HapticRumbleSupported`]<br>
    pub fn SDL_InitHapticRumble(haptic: *mut SDL_Haptic) -> ::core::primitive::bool;
}

extern "C" {
    /// Run a simple rumble effect on a haptic device.
    ///
    /// - `haptic`: the haptic device to play the rumble effect on.
    /// - `strength`: strength of the rumble to play as a 0-1 float value.
    /// - `length`: length of the rumble to play in milliseconds.
    /// - Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_InitHapticRumble`]<br>
    /// See also [`SDL_StopHapticRumble`]<br>
    pub fn SDL_PlayHapticRumble(
        haptic: *mut SDL_Haptic,
        strength: ::core::ffi::c_float,
        length: Uint32,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Stop the simple rumble on a haptic device.
    ///
    /// - `haptic`: the haptic device to stop the rumble effect on.
    /// - Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// This function is available since SDL 3.0.0.
    ///
    /// See also [`SDL_PlayHapticRumble`]<br>
    pub fn SDL_StopHapticRumble(haptic: *mut SDL_Haptic) -> ::core::primitive::bool;
}

/// The haptic structure used to identify an SDL haptic.
///
/// This struct is available since SDL 3.0.0.
///
/// See also [`SDL_OpenHaptic`]<br>
/// See also [`SDL_OpenHapticFromJoystick`]<br>
/// See also [`SDL_CloseHaptic`]<br>
#[repr(C)]
#[non_exhaustive]
pub struct SDL_Haptic {
    _opaque: [::core::primitive::u8; 0],
}
