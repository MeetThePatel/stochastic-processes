use crate::{Path, SimulatedPath, StochasticProcess};

use nalgebra::Dim;
use rand::{distributions::Distribution, thread_rng};
use statrs::distribution::Normal;

/// The [Ornstein-Uhlenbeck](https://en.wikipedia.org/wiki/Ornstein%E2%80%93Uhlenbeck_process) process.
///
/// This is a stochastic process given by the following stochastic differential equation:
/// $$ \textrm{d}x_t = \theta (\mu - x_t) \textrm{d}t + \sigma \textrm{d} W_t $$
/// where $\theta$, $\mu$, and $\sigma$ are parameters of the process and $W_t$ is a standard Brownian motion.

// TODO: Write test to make sure that OU process has correct distribution.

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

impl StochasticProcess for OrnsteinUhlenbeck {
    #[allow(non_snake_case)]
    fn dynamics(&self, x: f32, dt: f32, dW: f32) -> f32 {
        self.theta * (self.mu - x) * dt + self.sigma * dW
    }

    fn simulate(&self, n: usize, dt: f32, x_0: f32) -> SimulatedPath {
        let mut path = Path::zeros_generic(Dim::from_usize(1), Dim::from_usize(n + 1));
	path[0] = x_0;

        // TODO: Abstract the Brownian Motion to separate struct.
        let mut rng = thread_rng();
        let increments: Vec<f32> = match Normal::new(0.0, 1.0) {
            Ok(dist) => dist,
            Err(_) => panic!("Bad parameters."),
        }
        .sample_iter(&mut rng)
        .take(n)
        .map(|x| x as f32)
        .collect();

        for i in 1..n + 1 {
            path[i] = path[i - 1] + self.dynamics(path[i - 1], dt, increments[i - 1]);
        }

        SimulatedPath::new(n, dt, path)
    }
}
