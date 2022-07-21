//! Stochastic processes simluation toolkit.
//!
//! This crate provides utilities for simulating various stochastic processes.
//!
//! # Quick Start
//!
//! To create a process, call the `new` constructor for the desired process, and supply the
//! constructor with the required parameters. To simulate a process, simply call `simulate`
//! on the process.
//!
//! In the following example, an [Ornstein-Uhlenbeck](struct@OrnsteinUhlenbeck) is created with
//! $\mu = \theta = \sigma = 1$. The processes is simluated with $x_0 = 0$ for 10 time steps,
//! each time step 1 unit long. The path is stored inside of a [`SimulatedPath`].
//!
//! ```
//! use stochastic_processes::{OrnsteinUhlenbeck, SimluatedPath};
//!
//! let ou: OrnsteinUhlenbeck = OrnsteinUhlenbeck::new(1.0, 1.0, 1.0);
//! let sim: SimulatedPath = ou.simulate(10, 1.0, 0.0)
//!
//! println!("{}", sim.path);
//! ```

mod ornsteinuhlenbeck;
pub use self::ornsteinuhlenbeck::*;

mod cir;
pub use self::cir::*;

mod gbm;
pub use self::gbm::*;

use nalgebra::{Dynamic, Matrix, VecStorage, U1};

// TODO: Add documentation.
pub trait StochasticProcess {
    #[allow(non_snake_case)]
    // TODO: Add documentation.
    fn dynamics(&self, x: f32, dt: f32, dW: f32) -> f32;

    // TODO: Add documentation.
    fn simulate(&self, n: usize, dt: f32, x_0: f32) -> SimulatedPath;
}

// TODO: Add documentation.
pub type Path = Matrix<f32, U1, Dynamic, VecStorage<f32, U1, Dynamic>>;

/// Simluated Path.
///
/// This is a struct holding one realization of a path.

pub struct SimulatedPath {
    /// Number of time steps to simulate.
    pub n: usize,
    /// Size of time step.
    pub dt: f32,
    /// Simulated path.
    pub path: Path,
}

impl SimulatedPath {
    pub fn new(n: usize, dt: f32, path: Path) -> Self {
        Self { n, dt, path }
    }
}
