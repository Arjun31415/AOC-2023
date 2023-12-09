use std::collections::{HashMap, HashSet};

fn handle_input() -> String {
    let binding = include_str!("../inputs/day4/day4.in").to_string();
    let input = binding.trim_end();
    return input.to_string();
}
fn part1() {
    let mut input = handle_input();
    input = input.replace("Card ", "");
    let mut s = 0;
    for mut line in input.lines() {
        line = line.trim();
        line = &line[line.find(":").unwrap() + 1..];
        let w: Vec<HashSet<u32>> = line
            .split('|')
            .map(|s| s.trim())
            .map(|s| {
                s.split(' ')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<HashSet<u32>>()
            })
            .collect();
        let cnt = w[0].intersection(&w[1]).count();
        if cnt == 0 {
            continue;
        }
        s += 1 << (cnt - 1);
    }
    println!("{}", s);
}
fn part2() {
    let mut input = handle_input();
    input = input.replace("Card ", "");
    let mut s = 0;
    let mut f: HashMap<usize, i32> = HashMap::new();
    for (i, mut line) in input.lines().enumerate() {
        line = line.trim();
        line = &line[line.find(":").unwrap() + 1..];
        *f.entry(i).or_insert(0) += 1;
        let w: Vec<HashSet<u32>> = line
            .split('|')
            .map(|s| s.trim())
            .map(|s| {
                s.split(' ')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<HashSet<u32>>()
            })
            .collect();
        let cnt = w[0].intersection(&w[1]).count();
        for j in i + 1..i + cnt + 1 {
            *f.entry(j).or_insert(0) += f[&i];
        }
    }
    for (_, v) in &f {
        s += v;
    }
    dbg!(&f);
    println!("{}", s);
}
pub fn solve(x: bool) {
    if x {
        part2();
    } else {
        part1();
    }
}
