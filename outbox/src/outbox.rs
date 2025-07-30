//! Outbox is a UI framework that uses Core Graphics and Core Font to render UI elements efficiently.
//! It provides a simple and intuitive API for creating and managing UI components.
//! ## Features
//! - **Core Graphics**: Handles low-level drawing of shapes, shadows, and images.
//! - **Core Font**: Manages font importing and rasterization of fonts.
//! - **Text Rendering**: Supports advanced text rendering with custom styles and transformations.
//! - **UI Components**: Provides a set of UI components for building user interfaces.
//! - **Cross-Platform**: Designed to work across different platforms with minimal changes.

/// This module provides the main entry point for the Outbox UI framework.
pub mod app;
/// This module contains all the components that can be rendered in Outbox.
pub mod component;
/// This module contains the `Window` struct and related functionality for creating and managing windows.
pub mod event;
/// This module defines the `Renderable` trait and related types for rendering UI components.
pub mod renderable;
/// This module contains the `Window` struct and related functionality for creating and managing windows.
pub mod window;
