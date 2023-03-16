use mpi::datatype::Partition;
use mpi::topology::MergeOrder;
use mpi::traits::*;
use mpi::Count;
use std::env;
use std::process::Command;

fn main() {
    let (universe, _) = mpi::initialize_with_threading(mpi::Threading::Funneled)
        .expect("failed to initialize MPI runtime");
    let world = universe.world();

    // let mcomm = world.duplicate();
    let mcomm = if let Some(parent) = world.parent() {
        parent.merge(MergeOrder::High)
    } else {
        let cmd = Command::new(env::current_exe().unwrap());
        let inter_comm = world.process_at_rank(0).spawn(&cmd, 5).unwrap();
        inter_comm.merge(MergeOrder::Low)
    };

    let result = if mcomm.rank() == 0 {
        // The original data I would like to scatter between processes.
        // I cannot use UserDatatype::indexed here because I use scatter functionality.
        let data: Vec<Vec<_>> = (0..(mcomm.size() as usize))
            .map(|x| (0..(x + 2)).collect())
            .collect();

        let counts: Vec<_> = data.iter().map(|x| x.len() as Count).collect();
        let displs: Vec<_> = counts
            .iter()
            .scan(0 as Count, |state, x| {
                let next_value = *state;
                *state = *x + *state;
                Some(next_value)
            })
            .collect();
        let flat_data: Vec<_> = data.into_iter().flat_map(|x| x).collect();

        // Scatter counts first
        let mut count: Count = 0;
        mcomm
            .process_at_rank(0)
            .scatter_into_root(&counts[..], &mut count);

        // Then scatter the data
        let mut result = vec![0usize; count as usize];
        let data = Partition::new(&flat_data[..], counts, displs);
        mcomm
            .process_at_rank(0)
            .scatter_varcount_into_root(&data, &mut result);
        result
    } else {
        let mut count: Count = 0;
        mcomm.process_at_rank(0).scatter_into(&mut count);

        let mut result = vec![0usize; count as usize];
        mcomm.process_at_rank(0).scatter_varcount_into(&mut result);
        result
    };

    println!("rank {} result {:?}", mcomm.rank(), &result[..]);

    mcomm.barrier();
    println!("finished");
}
