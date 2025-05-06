use mpi::traits::Communicator;

fn main() {
    let universe = mpi::initialize().expect("Couldn't initialize universe!");
    let world = universe.world();
    println!("Hello from process number {}!", world.rank());
}
