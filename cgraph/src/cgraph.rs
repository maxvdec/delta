//! # Core Graphics
//! Core Graphics is the Library that powers Delta's rendering engine.
//! It provides a high-level API for creating and rendering 2D graphics, including shapes,
//! text, and images. Core Graphics is designed to be easy to use and efficient, making
//! it suitable for a wide range of applications, from simple games to complex graphics.
//! For now, it only supports macOS, but it can be extended to other platforms in the future.
#![deny(missing_docs)]

/// The module that contains the main window and application functionality for Core Graphics.
pub mod app;
/// The module that contains the object and primitive types used in Core Graphics.
pub mod object;
/// The module that contains the text rendering functionality for Core Graphics.
pub mod text;

/// The module that contains the renderer functionality for Core Graphics.
pub mod renderer;
mod utils;

#[cfg(target_os = "macos")]
/// The module that contains the macOS-specific functionality for Core Graphics.
pub mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;
