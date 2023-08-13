use proconio::{input, source::line::LineSource};
use std::{
    collections::HashSet,
    io::{stdin, BufReader, StdinLock},
    mem,
    process::exit,
};

fn transpose(temps: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut nxttemps = vec![vec![0; temps.len()]; temps.len()];
    for i in 0..temps.len() {
        for j in 0..temps.len() {
            nxttemps[j][i] = temps[i][j];
        }
    }
    return nxttemps;
}

fn linear_completion(_temps: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut temps = _temps.clone();
    let mut transposed = false;
    loop {
        let mut changed = false;
        for i in 0.._temps.len() {
            let mut v = Vec::<(i32, usize)>::new();
            for j in 0.._temps.len() {
                if temps[i][j] == -1 {
                    continue;
                }
                v.push((temps[i][j], j));
            }
            if v.len() > 1 && v.len() < _temps.len() {
                for x in 0..v.len() {
                    let diff = v[(x + 1) % v.len()].0 - v[x].0;
                    let nxtidx = v[(x + 1) % v.len()].1
                        + if v[(x + 1) % v.len()].1 > v[x].1 {
                            0
                        } else {
                            _temps.len()
                        };
                    let difflen = nxtidx - v[x].1;
                    for j in v[x].1 + 1..nxtidx {
                        temps[i][j % _temps.len()] =
                            v[x].0 + diff * (j - v[x].1) as i32 / difflen as i32;
                    }
                }
                changed = true;
            }
        }
        if !transposed && !changed {
            break;
        }
        let mut nxttemps = transpose(&temps);
        mem::swap(&mut temps, &mut nxttemps);
        transposed = !transposed;
    }
    return temps;
}

fn strategy1(
    source: &mut LineSource<BufReader<StdinLock<'_>>>,
    grid_size: usize,
    num_exit: usize,
    stdev: i32,
    exit_cells: Vec<(usize, usize)>,
) {
    println!("# strategy1");
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
    // 線形補間
    let temps_yoko = linear_completion(&temps);
    let temps_tate = transpose(&linear_completion(&transpose(&temps)));
    for i in 0..grid_size {
        for j in 0..grid_size {
            temps[i][j] = (temps_yoko[i][j] + temps_tate[i][j]) / 2;
        }
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
    for turn in 0..6 * num_exit {
        println!("{} {} {}", turn % num_exit, 0, 0);
        input! {
            from &mut *source,
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

fn strategy2(
    source: &mut LineSource<BufReader<StdinLock<'_>>>,
    grid_size: usize,
    num_exit: usize,
    stdev: i32,
    exit_cells: Vec<(usize, usize)>,
) {
    println!("# strategy2");
    let mut temps = vec![vec![0; grid_size]; grid_size];
    let center = (grid_size / 2, grid_size / 2);
    temps[center.0][center.1] = (6 * stdev).min(1000);

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
    for i in 0..num_exit - 1 {
        for j in 0..num_exit {
            if !remaining.contains(&j) {
                continue;
            }
            let mut cnt = 0;
            for _ in 0..3 {
                println!(
                    "{} {} {}",
                    i,
                    center.0 as i32 - exit_cells[j].0 as i32,
                    center.1 as i32 - exit_cells[j].1 as i32
                );
                input! {
                    from &mut *source,
                    measure_result: i32
                };
                if measure_result == -1 {
                    exit(0);
                }
                if measure_result > temps[center.0][center.1] / 2 {
                    cnt += 1;
                }
            }
            if cnt >= 2 {
                ans[i] = j;
                remaining.remove(&j);
                break;
            }
        }
    }
    ans[num_exit - 1] = *remaining.iter().next().unwrap();

    // output results
    println!("-1 -1 -1");
    for i in 0..num_exit {
        println!("{}", ans[i]);
    }
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

    if 1000 >= (num_exit * 2 + 1) as i32 * stdev {
        strategy1(&mut source, grid_size, num_exit, stdev, exit_cells);
    } else {
        strategy2(&mut source, grid_size, num_exit, stdev, exit_cells);
    }
}
