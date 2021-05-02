mod app;
mod ecs;
mod std_io_plugin;

pub use crate::app::{build_commands, match_commands, Pause};
pub use crate::std_io_plugin::ConsoleDebugPlugin;
