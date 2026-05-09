//! TOML configuration file parsing.

use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;

use cfd_core::CfdError;

/// Top-level simulation configuration.
#[derive(Debug, Deserialize)]
pub struct SimConfig {
    pub mesh: MeshConfig,
    pub physics: PhysicsConfig,
    #[serde(default)]
    pub boundary: HashMap<String, BoundaryConfig>,
    pub fluid: FluidConfig,
    pub output: OutputConfig,
}

#[derive(Debug, Deserialize)]
pub struct MeshConfig {
    pub path: String,
    #[serde(default = "default_mesh_format")]
    pub format: String,
}

fn default_mesh_format() -> String {
    "gmsh".to_string()
}

#[derive(Debug, Deserialize)]
pub struct PhysicsConfig {
    #[serde(default = "default_gas")]
    pub gas: String,
    #[serde(default = "default_rho_g")]
    pub rho_g: f64,
    #[serde(default = "default_mu_g")]
    pub mu_g: f64,
    #[serde(default = "default_epsilon")]
    pub epsilon: f64,
    #[serde(default)]
    pub ion_mobility: f64,
    #[serde(default)]
    pub ion_diffusion: f64,
}

fn default_gas() -> String {
    "air".to_string()
}
fn default_rho_g() -> f64 {
    1.225
}
fn default_mu_g() -> f64 {
    1.81e-5
}
fn default_epsilon() -> f64 {
    8.8541878128e-12
}

#[derive(Debug, Deserialize)]
pub struct BoundaryConfig {
    #[serde(default)]
    pub phi: Option<BcSpec>,
    #[serde(default)]
    pub ion_density: Option<BcSpec>,
    #[serde(default)]
    pub velocity: Option<BcSpec>,
}

#[derive(Debug, Deserialize)]
pub struct BcSpec {
    #[serde(rename = "type")]
    pub bc_type: String,
    #[serde(default)]
    pub value: f64,
}

#[derive(Debug, Deserialize)]
pub struct FluidConfig {
    #[serde(default = "default_fluid_model")]
    pub model: String,
    #[serde(default = "default_advection")]
    pub advection: String,
    #[serde(default = "default_pressure_solver")]
    pub pressure_solver: String,
    pub dt: f64,
    pub steps: usize,
    #[serde(default)]
    pub cfl: Option<f64>,
    #[serde(default)]
    pub max_time: Option<f64>,
}

fn default_fluid_model() -> String {
    "incompressible".to_string()
}
fn default_advection() -> String {
    "upwind".to_string()
}
fn default_pressure_solver() -> String {
    "cg".to_string()
}

#[derive(Debug, Deserialize)]
pub struct OutputConfig {
    #[serde(default = "default_output_format")]
    pub format: String,
    #[serde(default = "default_every")]
    pub every: usize,
    #[serde(default = "default_output_path")]
    pub path: String,
    #[serde(default)]
    pub fields: Vec<String>,
}

fn default_output_format() -> String {
    "vtu".to_string()
}
fn default_every() -> usize {
    10
}
fn default_output_path() -> String {
    "output/".to_string()
}

impl SimConfig {
    /// Load configuration from a TOML file.
    pub fn load(path: &Path) -> Result<Self, CfdError> {
        let content = std::fs::read_to_string(path)?;
        toml::from_str(&content).map_err(|e| CfdError::Config(format!("TOML parse error: {e}")))
    }
}
