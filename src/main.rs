use mpi::traits::{Communicator, Destination, Source};

fn main() {
    let universe = mpi::initialize().expect("Couldn't initialize universe!");
    let world = universe.world();
    println!("Hello from process number {}!", world.rank());

    let rank = world.rank(); // just to save some typing

    if rank == 0 { // if we're the first process in the chain
        println!("Process {} sending number 2.", rank);
        world.process_at_rank(rank + 1).send(&2);
    } else {
        // could also use world.any_process().receive();
        let (number, _): (i32, _) = world.process_at_rank(rank - 1).receive();

        // process our data
        let bigger = number * 2;
        println!("Process {rank} received number {number}, multiplied up to {bigger}.");

        // if we're not last, send it to the next process
        if rank != world.size() - 1 {
            println!("Process {} sending {}.", rank, bigger);
            world.process_at_rank(rank + 1).send(&bigger);
        }
    }
}
