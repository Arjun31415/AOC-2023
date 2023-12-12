use rayon::prelude::*;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
fn handle_input() -> String {
    let binding = include_str!("../inputs/day5/day5.in").to_string();
    let input = binding.trim_end();
    return input.to_string();
}
fn part1bf() {
    let input = handle_input();
    let mut maps: Vec<HashMap<u32, u32>> = vec![HashMap::new(); 7];
    let mut lines = input.lines();
    let seeds: Vec<u32> = lines.next().unwrap()[6..]
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    lines.next().unwrap();
    // dbg!(seeds);
    let mut k = 0;
    for line in lines {
        if line.trim().len() == 0 {
            // println!("new line");
            k += 1;
            continue;
        }
        if line.chars().next().unwrap().is_numeric() {
            let v: Vec<u32> = line
                .split(' ')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            for i in 0..v[2] {
                *maps[k].entry(v[1] + i).or_insert(v[1] + i) = v[0] + i;
            }
        }
    }
    // dbg!(&maps[0]);
    let mn = seeds
        .iter()
        .map(|x| {
            let mut t = x;
            for m in &maps {
                t = m.get(t).unwrap_or(t);
            }
            return t;
        })
        .min()
        .unwrap();
    println!("{}", mn);
}
fn part1() {
    let input = handle_input();
    let mut maps: Vec<Vec<Vec<u64>>> = vec![vec![]; 7];
    let mut lines = input.lines();
    let seeds: Vec<u64> = lines.next().unwrap()[6..]
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    lines.next().unwrap();
    let mut k = 0;
    for line in lines {
        if line.trim().len() == 0 {
            k += 1;
            continue;
        }
        if line.chars().next().unwrap().is_numeric() {
            let v = line
                .split(' ')
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            maps[k].push(v);
        }
    }
    let mut mn = u64::MAX;
    for s in seeds {
        let mut x = s;
        for m in &maps {
            for l in m {
                if x >= l[1] && x <= l[1] + l[2] - 1 {
                    x = l[0] + (x - l[1]);
                    break;
                }
            }
        }
        mn = mn.min(x);
    }
    println!("{}", mn);
}
fn part2() {
    let input = handle_input();
    let mut maps: Vec<Vec<Vec<u64>>> = vec![vec![]; 7];
    let mut lines = input.lines();
    let seeds: Vec<u64> = lines.next().unwrap()[6..]
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    lines.next().unwrap();
    let mut k = 0;
    for line in lines {
        if line.trim().len() == 0 {
            k += 1;
            continue;
        }
        if line.chars().next().unwrap().is_numeric() {
            let v = line
                .split(' ')
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            maps[k].push(v);
        }
    }
    let mn: Arc<Mutex<u64>> = Arc::new(Mutex::new(u64::MAX));
    for i in (0..seeds.len()).step_by(2) {
        (seeds[i]..seeds[i] + seeds[i + 1])
            .into_par_iter()
            .for_each(|s| {
                let mut x = s;
                for m in &maps {
                    for l in m {
                        if x >= l[1] && x <= l[1] + l[2] - 1 {
                            x = l[0] + (x - l[1]);
                            break;
                        }
                    }
                }
                let mut guard = mn.lock().unwrap();
                *guard = std::cmp::min(*guard, x);
                drop(guard);
            });
    }
    println!("{}", *mn.lock().unwrap());
}
pub fn solve(x: bool) {
    if x {
        part2();
    } else {
        part1();
    }
}
