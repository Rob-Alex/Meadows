mod physics;
use physics::fdtd::Simulator;

fn main() {

    let simulator = Simulator::new();
    println!("{:?}",simulator);

}