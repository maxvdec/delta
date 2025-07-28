//! Core Font is a library for importing and rasterizing fonts in Rust.
//! It provides functionality to load font files, extract glyph outlines, and render text geometries.
//! Core Font is designed to be efficient and easy to use, making it suitable for a wide range of applications, from simple text rendering to complex typography.
//! It is built on top of the `rustybuzz` and `lyon` libraries for text shaping and path rendering, respectively.
//! Core Font is currently focused on macOS, but it can be extended to other platforms in the future.
/// The module that contains the main font functionality for Core Font.
/// This module provides functions for loading fonts, shaping text, and rendering text geometries.
/// It is designed to be easy to use and efficient, making it suitable for a wide range of applications, from simple text rendering to complex typography.
pub mod font;
