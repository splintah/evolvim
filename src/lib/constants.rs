//! The collection of all constants used by `evolvim`.
//!
//! If you want to tinker around with the world of `evolvim` and try to get different creatures,
//! you have come to the right place!
//! Change the constants in this file to whatever you want, compile, and you're off to a world with brand new possibilities!
//! You should be able to find a description of what each constant does and estimate it's impact.
//! Have fun!
//!
//! TODO: transport all constants over to this file.

use super::*;

pub const SAFE_SIZE: f64 = 1.25;

/// used by creature.rs
pub const CREATURE_DENSITY: f64 = 1.0;

pub const ROCK_DENSITY: f64 = 5.0;

/// Used by creature.rs
pub const CREATURE_MIN_ENERGY: f64 = 1.2;

/// Used by creature.rs
pub const CREATURE_MAX_ENERGY: f64 = 2.0;

/// The default width when generating a new `Board`.
pub const DEFAULT_BOARD_WIDTH: usize = 100;

/// The default height when generating a new `Board`.
pub const DEFAULT_BOARD_HEIGHT: usize = 100;

/// The default size when generating a new `Board`.
///
/// NOTE: Don't change the value of this constant, change `DEFAULT_BOARD_WIDTH` and/or `DEFAULT_BOARD_HEIGHT` instead.
pub const DEFAULT_BOARD_SIZE: BoardSize = (DEFAULT_BOARD_WIDTH, DEFAULT_BOARD_HEIGHT);

/// The default minimum amount of creatures.
///
/// New random creatures will be generated if the population drops under this amount.
pub const DEFAULT_CREATURE_MINIMUM: usize = 60;

/// The amount of rocks in the world.
pub const DEFAULT_ROCK_AMOUNT: usize = 0;

/// The coldest it is going to get.
pub const DEFAULT_MIN_TEMP: f64 = -0.5;

/// The hottest it is going to get.
pub const DEFAULT_MAX_TEMP: f64 = 1.0;

/// Determines whether you start with "user control" or not.
///
/// Set to false to immediately let creatures loose in the world,
/// or set to true to stop creatures from moving and take control of them.
pub const START_IN_CONTROL: bool = false;

/// Used for terrain generation.
pub const DEFAULT_NOISE_STEP_SIZE: f64 = 0.1;