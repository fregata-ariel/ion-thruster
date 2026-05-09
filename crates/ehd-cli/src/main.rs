//! `ehd-sim` — CLI entry point for the EHD ion craft simulator.
//!
//! Wires together the CFD framework crates with the EHD physics module.

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use tracing_subscriber::EnvFilter;

use cfd_fields::SimState;
use cfd_io::{SimConfig, VtuWriter};
use cfd_mesh::gmsh;
use cfd_time::{OperatorSplitting, SimulationDriver};
use ehd_physics::{EhdConfig, EhdModule};

#[derive(Parser, Debug)]
#[command(name = "ehd-sim", about = "EHD ion craft simulator")]
struct Args {
    /// Path to the simulation TOML config file.
    #[arg(short, long, default_value = "simulation.toml")]
    config: PathBuf,

    /// Override mesh file path.
    #[arg(short, long)]
    mesh: Option<PathBuf>,

    /// Override output directory.
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let args = Args::parse();

    tracing::info!("ehd-sim starting");

    // Load config
    let config = SimConfig::load(&args.config).context("Failed to load config")?;

    // Load mesh
    let mesh_path = args
        .mesh
        .unwrap_or_else(|| PathBuf::from(&config.mesh.path));
    tracing::info!(?mesh_path, "Loading mesh");
    let mesh = gmsh::read_msh(&mesh_path).context("Failed to load mesh")?;

    tracing::info!(
        cells = mesh.n_cells,
        faces = mesh.n_faces,
        internal_faces = mesh.n_internal_faces,
        nodes = mesh.n_nodes,
        patches = mesh.boundary_patches.len(),
        "Mesh loaded"
    );

    // Create EHD physics module
    let ehd_config = EhdConfig {
        permittivity: config.physics.epsilon,
        ion_mobility: config.physics.ion_mobility,
        ion_diffusion: config.physics.ion_diffusion,
        gas_density: config.physics.rho_g,
        gas_viscosity: config.physics.mu_g,
        ..Default::default()
    };
    let ehd = EhdModule::new(ehd_config);

    // Initialize state
    let mut state = SimState::new();
    ehd.register_fields(&mut state.fields, &mesh);
    ehd.initialize(&mut state, &mesh);

    // Build time stepper
    let cfl = config.fluid.cfl.unwrap_or(0.5);
    let mut splitting = OperatorSplitting::new(cfl);
    for step in ehd.splitting_steps() {
        splitting.add_step(step);
    }

    // Create output writer
    let output_dir = args
        .output
        .unwrap_or_else(|| PathBuf::from(&config.output.path));
    let (scalar_fields, vector_fields) = ehd.output_fields();
    let mut writer = VtuWriter::new(&output_dir, scalar_fields, vector_fields);

    // Run simulation
    let mut driver = SimulationDriver {
        splitting,
        max_steps: config.fluid.steps,
        max_time: config.fluid.max_time.unwrap_or(f64::INFINITY),
        output_interval: config.output.every,
        fixed_dt: Some(config.fluid.dt),
    };

    driver
        .run(&mesh, &mut state, &mut writer)
        .context("Simulation failed")?;

    tracing::info!("ehd-sim complete");
    Ok(())
}
