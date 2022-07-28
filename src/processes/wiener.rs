use crate::processes::*;

/// The [Wiener](https://en.wikipedia.org/wiki/Wiener_process) process.
///
/// Standard Brownian Motion.
pub struct Wiener {}

impl Wiener {
    pub fn new() -> Self { Self {} }
}

impl StochasticProcess for Wiener {}

impl AutonomousStochasticProcess for Wiener {
    fn drift(&self, _x: f32) -> f32 { 0.0 }

    fn diffusion(&self, _x: f32) -> f32 { 1.0 }
}