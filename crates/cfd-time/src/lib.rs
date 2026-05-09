//! `cfd-time` — Time integration and operator splitting orchestration.
//!
//! Provides the `SplittingStep` trait for individual equation sub-steps,
//! `OperatorSplitting` for composing steps, and `SimulationDriver` for
//! running the main time loop.

use cfd_core::CfdError;
use cfd_fields::SimState;
use cfd_mesh::Mesh;

/// A single sub-step within operator splitting.
///
/// Physics modules provide concrete implementations (e.g., PoissonStep,
/// IonTransportStep, FluidStep). Each step advances one aspect of the
/// physics by `dt`, mutating the shared `SimState`.
pub trait SplittingStep: Send {
    /// Human-readable name for logging.
    fn name(&self) -> &str;

    /// Advance this sub-step by dt, mutating state in place.
    fn advance(&mut self, mesh: &Mesh, state: &mut SimState, dt: f64) -> Result<(), CfdError>;

    /// Estimate maximum stable dt for this sub-step.
    ///
    /// Return `f64::INFINITY` for unconditionally stable (implicit) steps.
    fn max_dt(&self, mesh: &Mesh, state: &SimState) -> f64;
}

/// Orchestrates a sequence of `SplittingStep`s per timestep.
pub struct OperatorSplitting {
    steps: Vec<Box<dyn SplittingStep>>,
    cfl: f64,
}

impl OperatorSplitting {
    pub fn new(cfl: f64) -> Self {
        Self {
            steps: Vec::new(),
            cfl,
        }
    }

    pub fn add_step(&mut self, step: Box<dyn SplittingStep>) {
        self.steps.push(step);
    }

    /// Compute global dt as `CFL * min(step.max_dt())`.
    pub fn compute_dt(&self, mesh: &Mesh, state: &SimState) -> f64 {
        let mut dt_min = f64::INFINITY;
        for step in &self.steps {
            let dt = step.max_dt(mesh, state);
            if dt < dt_min {
                dt_min = dt;
            }
        }
        self.cfl * dt_min
    }

    /// Run one full timestep: call each step in sequence.
    pub fn advance(&mut self, mesh: &Mesh, state: &mut SimState) -> Result<(), CfdError> {
        let dt = state.dt;
        for step in &mut self.steps {
            step.advance(mesh, state, dt)?;
        }
        Ok(())
    }
}

/// Trait for output writers, used by `SimulationDriver`.
pub trait FieldWriter: Send {
    fn write_frame(&mut self, mesh: &Mesh, state: &SimState, step: usize) -> Result<(), CfdError>;
}

/// Main simulation loop driver.
pub struct SimulationDriver {
    pub splitting: OperatorSplitting,
    pub max_steps: usize,
    pub max_time: f64,
    pub output_interval: usize,
    pub fixed_dt: Option<f64>,
}

impl SimulationDriver {
    /// Run the simulation from current state to completion.
    pub fn run(
        &mut self,
        mesh: &Mesh,
        state: &mut SimState,
        writer: &mut dyn FieldWriter,
    ) -> Result<(), CfdError> {
        tracing::info!(
            max_steps = self.max_steps,
            max_time = self.max_time,
            "Starting simulation"
        );

        // Write initial state
        writer.write_frame(mesh, state, 0)?;

        for step_num in 1..=self.max_steps {
            // Compute dt
            state.dt = match self.fixed_dt {
                Some(dt) => dt,
                None => self.splitting.compute_dt(mesh, state),
            };

            // Advance
            self.splitting.advance(mesh, state)?;

            state.time += state.dt;
            state.step = step_num;

            // Output
            if step_num % self.output_interval == 0 {
                tracing::info!(
                    step = step_num,
                    time = state.time,
                    dt = state.dt,
                    "Writing output frame"
                );
                writer.write_frame(mesh, state, step_num)?;
            }

            // Check termination
            if state.time >= self.max_time {
                tracing::info!(time = state.time, "Reached max_time, stopping");
                break;
            }
        }

        // Write final state if not already written
        if state.step % self.output_interval != 0 {
            writer.write_frame(mesh, state, state.step)?;
        }

        tracing::info!(
            total_steps = state.step,
            final_time = state.time,
            "Simulation complete"
        );

        Ok(())
    }
}
