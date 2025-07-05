//! # Terraria World Parser
//!
//! A simple library for parsing Terraria world files (.wld).
//!
//! ## Example
//!
//! ```rust
//! use terraria_world_parser::WorldFile;
//! use std::fs::File;
//!
//! // Parse a world file
//! let file = File::open("world.wld")?;
//! let world = WorldFile::parse(file)?;
//!
//! println!("World name: {}", world.name);
//! println!("World size: {}x{}", world.width, world.height);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod error;
pub mod header;
pub mod world;
pub mod parser;
pub mod types;

pub use error::ParseError;
pub use world::WorldFile;
pub use parser::WorldParser;

/// Result type used throughout the library
pub type Result<T> = std::result::Result<T, ParseError>;

