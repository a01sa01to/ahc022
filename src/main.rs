use proconio::{input, source::line::LineSource};
use std::{
    io::{stdin, BufReader},
    process::exit,
};

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
    let max_temp = 1000;

    for i in 0..grid_size {
        for j in 0..grid_size {
            let mut output = false;
            for x in 0..num_exit {
                if exit_cells[x] == (i, j) {
                    print!("{} ", max_temp * (x + 1) / num_exit);
                    output = true;
                    break;
                }
            }
            if !output {
                print!("0 ");
            }
        }
        println!("");
    }

    // measure
    let mut ans = vec![(0, 0); num_exit];
    let max_measure = 10000;
    for turn in 0..max_measure {
        println!("{} {} {}", turn % num_exit, 0, 0);
        input! {
            from &mut source,
            measure_result: i32
        };
        if measure_result == -1 {
            exit(0);
        }
        ans[turn % num_exit].0 += measure_result;
        ans[turn % num_exit].1 += 1;
    }

    // output
    println!("-1 -1 -1");
    for i in 0..num_exit {
        let measure = ans[i].0 as f64 / ans[i].1 as f64;
        let mut minim = (1e9, 0);
        for x in 0..num_exit {
            let temp = max_temp * (x + 1) / num_exit;
            let diff = (measure - temp as f64).abs();
            if minim.0 > diff {
                minim = (diff, x);
            }
        }
        println!("{}", minim.1);
    }
}
