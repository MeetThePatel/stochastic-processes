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
//! In the following example, a [Geometric Brownian motion](struct@gbm::GeometricBrownianMotion) is created with
//! $\mu = \sigma = 1$. The processes is simluated using the
//! [Euler-Maruyama](trait@EulerMaruyama::EulerMaruyama) method. The path is stored inside of a
//! [`SimulatedPath`]. Finally, the path is exported to a pickle file (for use in Python).
//!
//! ```
//! use stochastic_processes::prelude::*;
//!
//! let proc = GeometricBrownianMotion {
//!     mu: 1.0,
//!     sigma: 1.0,
//! };
//!
//! let sim = proc.run_euler_maruyama(1.0, 0.0, 1.0, 20);
//! let _ = export_to_pickle(sim, "/tmp/test.pickle").unwrap();
//! ```

pub mod processes;

#[cfg(any(feature = "py", feature = "json"))]
pub mod export;

/// A convenience module appropriate for glob imports.
pub mod prelude {
    pub use crate::SimulatedPath;
    pub use crate::processes::*;

    #[cfg(feature = "py")]
    pub use crate::export::py::*;

    #[cfg(feature = "json")]
    pub use crate::export::json::*;
}

/// Representation of a simluated path.
///
/// For $n$ time-steps, a simulated path is a $(n+1) \times 2$ matrix, where the first colum
/// holds the time and the second column holds the process value.
pub type SimulatedPath = nalgebra::Matrix<
    f32,
    nalgebra::Dynamic,
    nalgebra::U2,
    nalgebra::VecStorage<f32, nalgebra::Dynamic, nalgebra::U2>,
>;
