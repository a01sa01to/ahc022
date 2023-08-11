use proconio::{input, source::line::LineSource};
use std::io::{stdin, BufReader};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        grid_size: u32,
        num_exit: usize,
        stdev: u32,
        exit_cells: [(u32, u32); num_exit]
    };

    for i in 0..grid_size {
        for j in 0..grid_size {
            print!("0 ");
        }
        println!("");
    }

    // output
    println!("-1 -1 -1");
    for _ in 0..num_exit {
        println!("0");
    }
}
