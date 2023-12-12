use std::collections::HashSet;

use rayon::iter::{FromParallelIterator, IndexedParallelIterator, IntoParallelRefIterator};

fn handle_input() -> String {
    let binding = include_str!("../inputs/day11/day11.in").to_string();
    let input = binding.trim_end();
    return input.to_string();
}
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
fn generic_solve(expansion_rate: i64) {
    let input = handle_input();
    let b: Vec<Vec<char>> = input
        .lines()
        .map(|q| q.chars().collect::<Vec<char>>())
        .collect();
    let mut g = vec![];
    for (i, l) in b.iter().enumerate() {
        for (j, ch) in l.iter().enumerate() {
            if *ch == '#' {
                g.push((i, j));
            }
        }
    }
    let mut wr = vec![0 as i64; b.len()];
    let mut wc = vec![0 as i64; b[0].len()];

    for (i, r) in b.iter().enumerate() {
        if HashSet::<&char>::from_par_iter(r.par_iter()).len() == 1 {
            wr[i] = expansion_rate;
        } else {
            wr[i] = 1
        };
    }
    for (j, c) in transpose(b).iter().enumerate() {
        if HashSet::<&char>::from_par_iter(c.par_iter()).len() == 1 {
            wc[j] = expansion_rate;
        } else {
            wc[j] = 1
        };
    }
    for i in 1..wr.len() {
        wr[i] += wr[i - 1];
    }
    for j in 1..wc.len() {
        wc[j] += wc[j - 1];
    }
    let mut ans = 0;
    for i in 0..g.len() {
        for j in i + 1..g.len() {
            ans += (wr[g[i].0] - wr[g[j].0]).abs() + (wc[g[i].1] - wc[g[j].1]).abs();
        }
    }
    println!("{}", ans);
}

pub fn solve(x: bool) {
    if x {
        generic_solve(1000000);
    } else {
        generic_solve(2);
    }
}

