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
        grid_size: usize,
        num_exit: usize,
        stdev: i32,
        exit_cells: [(usize, usize); num_exit]
    };

    let mut temps = vec![vec![-1; grid_size]; grid_size];
    let mut tem = 2 * stdev;
    // ふつうに埋めるパート
    for i in 0..grid_size {
        for j in 0..grid_size {
            for x in exit_cells.iter() {
                if (i, j) == *x {
                    temps[i][j] = tem;
                    tem += 4 * stdev;
                }
            }
        }
    }
    // もし誤差が出るようであれば全部 0
    if tem > 1000 {
        for i in 0..grid_size {
            for j in 0..grid_size {
                temps[i][j] = 0;
            }
        }
    }
    // 線形補間
    loop {
        let mut changed = false;
        for i in 0..grid_size {
            let mut v = Vec::<(i32, usize)>::new();
            for j in 0..grid_size {
                if temps[i][j] == -1 {
                    continue;
                }
                v.push((temps[i][j], j));
            }
            if v.len() > 1 {
                for x in 0..v.len() {
                    let diff = v[(x + 1) % v.len()].0 - v[x].0;
                    let nxtidx = v[(x + 1) % v.len()].1
                        + if v[(x + 1) % v.len()].1 > v[x].1 {
                            0
                        } else {
                            grid_size
                        };
                    let difflen = nxtidx - v[x].1;
                    for j in v[x].1 + 1..nxtidx {
                        temps[i][j % grid_size] =
                            v[x].0 + diff * (j - v[x].1) as i32 / difflen as i32;
                    }
                }
                changed = true;
            }
        }
        break;
    }

    // output temps
    for i in 0..grid_size {
        for j in 0..grid_size {
            print!("{} ", temps[i][j].min(1000).max(0));
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
        // let mut ans = (0.0, 0);
        // for x in 0..num_exit {
        //     let mut prob = 1.0;
        //     let temp = max_temp * (x + 1) / num_exit;
        //     for j in 0..measure_res[i].len() {
        //         let diff = (measure_res[i][j] - temp as i32) as f64;
        //         prob *= (-(diff * diff) / (2.0 * (stdev * stdev))).exp();
        //     }
        //     if prob > ans.0 {
        //         ans = (prob, x);
        //     }
        // }
        // eprintln!("{} {}", ans.0, ans.1);
        println!("{}", 0);
    }
}
