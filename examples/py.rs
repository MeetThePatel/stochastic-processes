use stochastic_processes::prelude::*;

#[cfg(feature = "py")]
use stochastic_processes::export::py::export_to_pickle;

fn main() {
    let process = GeometricBrownianMotion::new(0.0, 1.0);
    let path = process.run_euler_maruyama(1.0, 0.0, 1.0, 10);

    #[cfg(feature = "py")]
    let _ = export_to_pickle(path, "/tmp/test.pickle");
}
