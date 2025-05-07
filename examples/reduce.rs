use mpi::{collective::SystemOperation, traits::{Communicator, Destination, Root, Source}};

fn main() {
    let universe = mpi::initialize().expect("Couldn't initialize universe!");
    let world = universe.world();
    println!("Hello from process number {}!", world.rank());

    let rank = world.rank(); // just to save some typing

    if rank == 0 { // if we're the first process in the chain
        println!("Process {} sending number 2.", rank);
        world.process_at_rank(rank + 1).send(&2u32);

        let mut total: u32 = 0;

        // sum and receive values (gather)
        world.process_at_rank(0).reduce_into_root(&2, &mut total, SystemOperation::sum());
        println!("Total is {}", total);
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

        world.process_at_rank(0).reduce_into(&bigger, SystemOperation::sum());
    }
}