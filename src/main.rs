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
    let stdev = stdev as f64;
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
    let mut measure_res = vec![Vec::<i32>::new(); num_exit];
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
        measure_res[turn % num_exit].push(measure_result);
    }

    // output
    println!("-1 -1 -1");
    for i in 0..num_exit {
        let mut ans = (0.0, 0);
        for x in 0..num_exit {
            let mut prob = 1.0;
            let temp = max_temp * (x + 1) / num_exit;
            for j in 0..measure_res[i].len() {
                let diff = (measure_res[i][j] - temp as i32) as f64;
                prob *= (-(diff * diff) / (2.0 * (stdev * stdev))).exp();
            }
            if prob > ans.0 {
                ans = (prob, x);
            }
        }
        eprintln!("{} {}", ans.0, ans.1);
        println!("{}", ans.1);
    }
}
