use proconio::{input, source::line::LineSource};
use std::{
    io::{stdin, BufReader},
    mem,
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

    let mut exit_cells_ordered = exit_cells.clone();
    exit_cells_ordered.sort_by(|a, b| {
        ((a.0 as i32 - grid_size as i32 / 2).abs() + (a.1 as i32 - grid_size as i32 / 2).abs()).cmp(
            &((b.0 as i32 - grid_size as i32 / 2).abs()
                + (b.1 as i32 - grid_size as i32 / 2).abs()),
        )
    });

    let mut temps = vec![vec![-1; grid_size]; grid_size];
    let mut tem = stdev;
    // ふつうに埋めるパート
    for (i, j) in exit_cells_ordered.iter() {
        temps[*i][*j] = tem;
        tem += 2 * stdev;
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
    let mut transposed = false;
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
            if v.len() > 1 && v.len() < grid_size {
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
        if !transposed && !changed {
            break;
        }
        let mut nxttemps = vec![vec![0; grid_size]; grid_size];
        for i in 0..grid_size {
            for j in 0..grid_size {
                nxttemps[j][i] = temps[i][j];
            }
        }
        mem::swap(&mut temps, &mut nxttemps);
        transposed = !transposed;
    }

    // output temps
    for i in 0..grid_size {
        for j in 0..grid_size {
            print!("{} ", temps[i][j].min(1000).max(0));
        }
        println!("");
    }

    if tem > 1000 {
        println!("-1 -1 -1");
        for _ in 0..num_exit {
            println!("0");
        }
        return;
    }

    // measure
    let mut measure_res = vec![Vec::<i32>::new(); num_exit];
    for turn in 0..6 * num_exit {
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
            let temp = temps[exit_cells[x].0][exit_cells[x].1];
            for j in 0..measure_res[i].len() {
                let diff = (measure_res[i][j] - temp as i32) as f64;
                prob *= (-(diff * diff) / (2 * (stdev * stdev)) as f64).exp();
            }
            if prob > ans.0 {
                ans = (prob, x);
            }
        }
        eprintln!("{} {}", ans.0, ans.1);
        println!("{}", ans.1);
    }
}
