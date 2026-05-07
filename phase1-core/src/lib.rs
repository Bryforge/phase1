#![forbid(unsafe_code)]
//! Phase1 reusable core API.
//!
//! This crate exposes stable, shell-independent Phase1 building blocks for
//! Base1 integration, automation, tests, and AI-assisted project management.
//! The interactive `phase1` binary remains the primary application crate.
//!
//! AI-NOTE: Keep this crate free of terminal input loops and host-specific UI.
//! Core code should be deterministic, testable, and safe to call from tools.

#[path = "../../src/arena.rs"]
pub mod arena;
pub mod history;
#[path = "../../src/kernel.rs"]
pub mod kernel;
#[path = "../../src/ops_log.rs"]
pub mod ops_log;
#[path = "../../src/policy.rs"]
pub mod policy;
#[path = "../../src/registry.rs"]
pub mod registry;
#[path = "../../src/text.rs"]
pub mod text;

pub use kernel::{
    AuditLog, Kernel, PcieDevice, PcieManager, ProcessState, Scheduler, SimProcess, Vfs, VfsNode,
};
pub use registry::{
    capabilities_report, canonical_name, command_map, completions, lookup, man_page, CommandSpec,
    CATEGORIES, COMMANDS,
};
