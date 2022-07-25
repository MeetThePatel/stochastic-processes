use crate::processes::*;

// TODO: Write test to make sure that OU process has correct distribution.

/// The [Geometric Brownian motion](https://en.wikipedia.org/wiki/Geometric_Brownian_motion) process.
///
/// This is a stochastic process given by the following stochastic differential equation:
/// $$ \textrm{d}x_t = \mu x_t \textrm{d} t + \sigma x_t \textrm{d} W_t $$
/// where $\theta$, $\mu$, and $\sigma$ are parameters of the process and $W_t$ is a standard Brownian motion.

pub struct GeometricBrownianMotion {
    /// $\mu$ is the (percentage) drift.
    pub mu: f32,

    /// $\sigma$ is the (percentage) volatility.
    pub sigma: f32,
}

impl GeometricBrownianMotion {
    /// Create a new Geometric Brownian Motion process.
    pub fn new(mu: f32, sigma: f32) -> Self {
        Self { mu, sigma }
    }
}

impl StochasticProcess for GeometricBrownianMotion {}

impl AutonomousStochasticProcess for GeometricBrownianMotion {
    fn drift(&self, x: f32) -> f32 {
        self.mu * x
    }

    fn diffusion(&self, x: f32) -> f32 {
        self.sigma * x
    }
}