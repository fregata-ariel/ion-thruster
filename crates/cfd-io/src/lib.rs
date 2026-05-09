//! `cfd-io` — Input/Output for the CFD framework.
//!
//! - VTU output for ParaView visualization
//! - TOML configuration file parsing
//! - PVD time series collection files

pub mod config;
pub mod vtu;

pub use config::SimConfig;
pub use vtu::VtuWriter;
