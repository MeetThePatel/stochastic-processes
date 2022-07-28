use crate::processes::*;

// TODO: Write test to make sure that OU process has correct distribution.

/// The [Ornstein-Uhlenbeck](https://en.wikipedia.org/wiki/Ornstein%E2%80%93Uhlenbeck_process) process.
///
/// This is a stochastic process given by the following stochastic differential equation:
/// $$ \textrm{d}x_t = \theta (\mu - x_t) \textrm{d}t + \sigma \textrm{d} W_t $$
/// where $\theta$, $\mu$, and $\sigma$ are parameters of the process and $W_t$ is a standard Brownian motion.

pub struct OrnsteinUhlenbeck {
    /// $\theta$ is the speed of reversion.
    pub theta: f32,

    /// $\mu$ is the long-term mean.
    pub mu: f32,

    /// $\sigma$ is the instantaneous volatility.
    pub sigma: f32,
}

impl OrnsteinUhlenbeck {
    /// Create a new Ornstein-Uhlenbeck process.
    pub fn new(theta: f32, mu: f32, sigma: f32) -> Self {
        Self { theta, mu, sigma }
    }
}

impl StochasticProcess for OrnsteinUhlenbeck {}

impl AutonomousStochasticProcess for OrnsteinUhlenbeck {
    fn drift(&self, x: f32) -> f32 {
        self.theta * (self.mu - x)
    }

    fn diffusion(&self, _x: f32) -> f32 {
        self.sigma
    }
}
