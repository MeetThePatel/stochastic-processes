use stochastic_processes::prelude::*;

fn main() {
    let proc = GeometricBrownianMotion::new(1.0, 1.0);

    let sim = proc.run_euler_maruyama(1.0, 0.0, 1.0, 20);

    println!("{:#?}", sim.data);
}
