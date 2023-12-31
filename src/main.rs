use libm::erfc;
use proconio::{input, source::line::LineSource};
use std::{
    collections::HashSet,
    io::{stdin, BufReader, StdinLock},
};

fn measure(i: usize, x: i32, y: i32, source: &mut LineSource<BufReader<StdinLock<'_>>>) -> i32 {
    static mut COUNT: usize = 0;
    unsafe { COUNT += 1 };
    if unsafe { COUNT } > 10000 {
        return -1;
    }
    println!("{} {} {}", i, x, y);
    input! {
        from &mut *source,
        measure_result: i32
    };
    return measure_result;
}

// P(X <= x)
fn prob(x: f64, stdev: i32) -> f64 {
    erfc(-x / (stdev as f64 * 2.0_f64.sqrt())) / 2.0
}

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        grid_size: usize,
        num_exit: usize,
        stdev: i32,
        exit_cells: [(usize, usize); num_exit]
    };

    let mut temps = vec![vec![0; grid_size]; grid_size];
    let center = (grid_size / 2, grid_size / 2);
    temps[center.0][center.1] = (8 * stdev).min(1000);

    // output temps
    for i in 0..grid_size {
        for j in 0..grid_size {
            print!("{} ", temps[i][j].min(1000).max(0));
        }
        println!("");
    }

    // measure
    let ans_undecided = 10000000;
    let mut ans = vec![ans_undecided; num_exit];
    let mut remaining = HashSet::<usize>::new();
    for i in 0..num_exit {
        remaining.insert(i);
    }

    let mut ordered_exitidx = (0..num_exit).collect::<Vec<usize>>();
    // let mut perm = (0..num_exit).collect::<Vec<usize>>();
    ordered_exitidx.sort_by(|a, b| {
        ((exit_cells[*a].0 as i32 - center.0 as i32).abs()
            + (exit_cells[*a].1 as i32 - center.1 as i32).abs())
        .cmp(
            &((exit_cells[*b].0 as i32 - center.0 as i32).abs()
                + (exit_cells[*b].1 as i32 - center.1 as i32).abs()),
        )
    });

    for i in 0..num_exit {
        for j in 0..num_exit {
            if !remaining.contains(&j) {
                continue;
            }
            let one_acceptance = 0.999;
            let zero_acceptance = 0.90;
            let mut percentage_one = 0.35;
            while percentage_one < one_acceptance && (1.0 - percentage_one) < zero_acceptance {
                let measure_result = measure(
                    j,
                    center.0 as i32 - exit_cells[ordered_exitidx[i]].0 as i32,
                    center.1 as i32 - exit_cells[ordered_exitidx[i]].1 as i32,
                    &mut source,
                );
                if measure_result == -1 {
                    break;
                }
                let percentage_zero = 1.0 - percentage_one;
                let t = temps[center.0][center.1];
                let prob_one = if measure_result >= t {
                    prob(0.5, stdev)
                } else if measure_result == 0 {
                    prob(-t as f64 + 0.5, stdev)
                } else {
                    prob((measure_result - t) as f64 + 0.5, stdev)
                        - prob((measure_result - t) as f64 - 0.5, stdev)
                };
                let prob_zero = if measure_result == 0 {
                    prob(0.5, stdev)
                } else if measure_result >= t {
                    prob(-t as f64 + 0.5, stdev)
                } else {
                    prob(-measure_result as f64 + 0.5, stdev)
                        - prob(-measure_result as f64 - 0.5, stdev)
                };
                let sum = percentage_one * prob_one + percentage_zero * prob_zero;
                percentage_one = percentage_one * prob_one / sum;
            }
            if percentage_one > one_acceptance {
                ans[j] = ordered_exitidx[i];
                remaining.remove(&j);
                break;
            }
        }
    }

    if remaining.len() > 0 {
        let mut remaining = HashSet::<usize>::new();
        for i in 0..num_exit {
            remaining.insert(i);
        }
        for i in 0..num_exit {
            if ans[i] != ans_undecided {
                remaining.remove(&i);
            }
        }
        eprintln!("remaining: {:?}", remaining);
        let rem = remaining.iter().next().unwrap().clone();
        for i in 0..num_exit {
            if ans[i] == ans_undecided {
                ans[i] = rem;
            }
        }
    }

    // output results
    println!("-1 -1 -1");
    for i in 0..num_exit {
        println!("{}", ans[i]);
    }
}
