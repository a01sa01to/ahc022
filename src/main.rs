use libm::erfc;
use proconio::{input, source::line::LineSource};
use rand::seq::SliceRandom;
use std::{
    collections::HashSet,
    f64::consts::PI,
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

fn sqr<T>(x: T) -> T
where
    T: std::ops::Mul<Output = T> + Copy,
{
    x * x
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
    let mut ans = vec![0; num_exit];
    let mut remaining = HashSet::<usize>::new();
    for i in 0..num_exit {
        remaining.insert(i);
    }

    let mut rng = rand::thread_rng();
    let mut ordered_exitidx = (0..num_exit).collect::<Vec<usize>>();
    let mut perm = (0..num_exit).collect::<Vec<usize>>();
    ordered_exitidx.sort_by(|a, b| {
        ((exit_cells[*a].0 as i32 - center.0 as i32).abs()
            + (exit_cells[*a].1 as i32 - center.1 as i32).abs())
        .cmp(
            &((exit_cells[*b].0 as i32 - center.0 as i32).abs()
                + (exit_cells[*b].1 as i32 - center.1 as i32).abs()),
        )
    });

    let erfcval = erfc(temps[center.0][center.1] as f64 / (stdev as f64 * 2.0_f64.sqrt())) / 2.0;

    for i in 0..num_exit {
        perm.shuffle(&mut rng);
        for _j in 0..num_exit {
            let j = perm[_j];
            if !remaining.contains(&j) {
                continue;
            }
            let acceptance = 0.995;
            let mut percentage_one = 0.5;
            while (percentage_one - 0.5_f64).abs() < (acceptance - 0.5_f64).abs() {
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
                    0.5
                } else if measure_result == 0 {
                    erfcval
                } else {
                    (-sqr(measure_result - t) as f64 / (2 * sqr(stdev)) as f64).exp()
                        / (2.0 * PI * sqr(stdev) as f64).sqrt()
                };
                let prob_zero = if measure_result == 0 {
                    0.5
                } else if measure_result >= t {
                    erfcval
                } else {
                    (-sqr(measure_result) as f64 / (2 * sqr(stdev)) as f64).exp()
                        / (2.0 * PI * sqr(stdev) as f64).sqrt()
                };
                let sum = percentage_one * prob_one + percentage_zero * prob_zero;
                percentage_one = percentage_one * prob_one / sum;
                // eprintln!("{} {} {}", percentage_one, prob_one, measure_result);
            }
            if percentage_one > acceptance {
                ans[j] = ordered_exitidx[i];
                remaining.remove(&j);
                break;
            }
        }
    }

    // output results
    println!("-1 -1 -1");
    for i in 0..num_exit {
        println!("{}", ans[i]);
    }
}
