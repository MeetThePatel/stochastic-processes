use crate::processes::*;

// TODO: Write test to make sure that the CIR process has the correct distribution.

/// The [Cox-Ingersoll-Ross (CIR)](https://en.wikipedia.org/wiki/Cox%E2%80%93Ingersoll%E2%80%93Ross_model) process.
///
/// This is a stochastic process given by the following stochastic differential equation:
/// $$ \textrm{d}x_t = \theta (\mu - x_t) \textrm{d}t + \sigma \sqrt{x_t} \textrm{d} W_t $$
/// where $\theta$, $\mu$, and $\sigma$ are parameters of the process and $W_t$ is a standard Brownian motion.
pub struct CIR {
    /// $\theta$ is the speed of reversion.
    pub theta: f32,

    /// $\mu$ is the long-term mean.
    pub mu: f32,

    /// $\sigma$ is the instantaneous volatility.
    pub sigma: f32,
}

impl CIR {
    /// Create a new CIR process.
    pub fn new(theta: f32, mu: f32, sigma: f32) -> Self {
        Self { theta, mu, sigma }
    }
}

impl StochasticProcess for CIR {}

impl AutonomousStochasticProcess for CIR {
    fn drift(&self, x: f32) -> f32 {
        self.theta * (self.mu - x)
    }

    fn diffusion(&self, x: f32) -> f32 {
        self.sigma * x.sqrt()
    }
}