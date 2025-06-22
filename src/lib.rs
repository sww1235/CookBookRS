//! Library functions of `CookBook`
//!

/// internal datatypes used in Cookbook
pub mod datatypes;

/// TUI and application setup and configuration
#[cfg(feature = "tui")]
pub mod tui;

/// Web Gui resources
#[cfg(feature = "wgui")]
pub mod wgui;
