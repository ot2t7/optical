//! Utilities for networking with Minecraft's specifications.
//!
//! This module only contains items regarding TCP, accepting clients, and seperating
//! the incoming buffer into packets. Most items here provide message-based responses
//! through [`channels`]. This design was chosen to be easy usable with non-async code
//! like ECS systems.
//!
//! [`channels`]: std::sync::mpsc

mod listener;
pub use listener::*;
