use std::collections::{HashMap, HashSet};

fn handle_input() -> String {
    let binding = include_str!("../inputs/day3/day3.in").to_string();
    let input = binding.trim_end();
    return input.to_string();
}
fn part1() {
    let input = handle_input();
    let mut f: HashMap<(usize, usize), u32> = HashMap::new();
    let mut leader: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let a: Vec<Vec<char>> = input
        .lines()
        .map(|f| {
            let mut x: Vec<char> = f.chars().collect();
            x.push('.');
            return x;
        })
        .collect();
    for i in 0..a.len() {
        let mut num = 0;
        let mut indices: Vec<usize> = vec![];
        for j in 0..a[i].len() {
            let ch = a[i][j];
            if ch.is_ascii_digit() {
                num = num * 10 + ch.to_digit(10).unwrap();
                indices.push(j);
            } else if indices.len() > 0 {
                for k in &indices {
                    *leader.entry((i, *k)).or_insert((i, *k)) = (i, indices[0]);
                }
                *f.entry((i, indices[0])).or_insert(0) = num;
                indices.clear();
                num = 0;
            }
        }
    }
    let mut s = 0;
    let di: [i32; 8] = [0, 0, 1, -1, 1, 1, -1, -1];
    let dj: [i32; 8] = [1, -1, 0, 0, 1, -1, 1, -1];
    // dbg!(a);
    for i in 0..a.len() {
        for j in 0..a[i].len() {
            let ch = a[i][j];
            if ch.is_ascii_digit() || ch == '.' {
                continue;
            }
            for k in 0..di.len() {
                let idx_i = i as i32 + di[k];
                let idx_j = j as i32 + dj[k];
                // dbg!((idx_i,idx_j));
                if idx_i < 0
                    || idx_j < 0
                    || idx_i >= a.len() as i32
                    || idx_j >= a[idx_i as usize].len() as i32
                    || !a[idx_i as usize][idx_j as usize].is_ascii_digit()
                // || !leader.contains_key(&(idx_i as usize, idx_j as usize))
                {
                    continue;
                }
                dbg!((idx_i, idx_j, a[idx_i as usize][idx_j as usize]));
                let leader_cell = &leader[&(idx_i as usize, idx_j as usize)];
                if !f.contains_key(leader_cell) {
                    continue;
                }
                // dbg!(leader_cell);
                s += f[leader_cell];
                // *f.entry(*leader_cell).or_insert(0) = 0;
                f.remove(leader_cell);
            }
        }
    }
    println!("{}", s);
    dbg!(&f);
}
fn part2() {
    let input = handle_input();
    let mut f: HashMap<(usize, usize), u32> = HashMap::new();
    let mut leader: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let a: Vec<Vec<char>> = input
        .lines()
        .map(|f| {
            let mut x: Vec<char> = f.chars().collect();
            x.push('.');
            return x;
        })
        .collect();
    for i in 0..a.len() {
        let mut num = 0;
        let mut indices: Vec<usize> = vec![];
        for j in 0..a[i].len() {
            let ch = a[i][j];
            if ch.is_ascii_digit() {
                num = num * 10 + ch.to_digit(10).unwrap();
                indices.push(j);
            } else if indices.len() > 0 {
                for k in &indices {
                    *leader.entry((i, *k)).or_insert((i, *k)) = (i, indices[0]);
                }
                *f.entry((i, indices[0])).or_insert(0) = num;
                indices.clear();
                num = 0;
            }
        }
    }
    let mut s = 0;
    let di: [i32; 8] = [0, 0, 1, -1, 1, 1, -1, -1];
    let dj: [i32; 8] = [1, -1, 0, 0, 1, -1, 1, -1];
    // dbg!(a);
    for i in 0..a.len() {
        for j in 0..a[i].len() {
            let ch = a[i][j];
            if ch != '*' {
                continue;
            }
            let mut gear_ratio:u64 = 1;
            let mut adjs:HashSet<(usize,usize)> = HashSet::new();
            for k in 0..di.len() {
                let idx_i = i as i32 + di[k];
                let idx_j = j as i32 + dj[k];
                // dbg!((idx_i,idx_j));
                if idx_i < 0
                    || idx_j < 0
                    || idx_i >= a.len() as i32
                    || idx_j >= a[idx_i as usize].len() as i32
                    || !a[idx_i as usize][idx_j as usize].is_ascii_digit()
                {
                    continue;
                }
                // dbg!((idx_i, idx_j, a[idx_i as usize][idx_j as usize]));
                let leader_cell = &leader[&(idx_i as usize, idx_j as usize)];
                if !f.contains_key(leader_cell) {
                    continue;
                }
                // dbg!(leader_cell);
                adjs.insert(*leader_cell);
                // *f.entry(*leader_cell).or_insert(0) = 0;
                // f.remove(leader_cell);
            }
            // dbg!((i,j,&adjs,gear_ratio));
            if adjs.len() == 2 {
                for g in adjs{
                    gear_ratio *=f[&g] as u64;
                }
                s+=gear_ratio;
            }
        }
    }
    println!("{}", s);
    // dbg!(&f);
}

pub fn solve(x: bool) {
    if x {
        part2();
    } else {
        part1();
    }
}
