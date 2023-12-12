use std::collections::{HashMap, VecDeque};

fn handle_input() -> String {
    let binding = include_str!("../inputs/day10/day10.in").to_string();
    let input = binding.trim_end();
    return input.to_string();
}
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Dir {
    North,
    South,
    East,
    West,
}

fn dir2coords(i: usize, j: usize, d: Dir) -> (usize, usize) {
    match d {
        Dir::North => (i - 1, j),
        Dir::South => (i + 1, j),
        Dir::East => (i, j + 1),
        Dir::West => (i, j - 1),
    }
}
fn coords2dir(i: usize, j: usize, p: usize, q: usize) -> Dir {
    match (p as i32 - i as i32, q as i32 - j as i32) {
        (-1, 0) => Dir::North,
        (1, 0) => Dir::South,
        (0, 1) => Dir::East,
        (0, -1) => Dir::West,
        _ => panic!("No such direction"),
    }
}
fn find_element_index<T: PartialEq>(matrix: &[Vec<T>], target: &T) -> Option<(usize, usize)> {
    for (i, row) in matrix.iter().enumerate() {
        if let Some(j) = row.iter().position(|element| element == target) {
            // 1 based position, i.e start from 1,2,3...n instead of 0,1,2...n-1
            return Some((i + 1, j + 1));
        }
    }
    None
}

fn part1() -> Vec<Vec<u32>> {
    let input = handle_input();
    let lines: Vec<Vec<_>> = input.lines().map(|s| s.chars().collect()).collect();
    let pipes_map: HashMap<char, [Option<Dir>; 4]> = HashMap::from([
        ('|', [Some(Dir::North), Some(Dir::South), None, None]),
        ('-', [Some(Dir::East), Some(Dir::West), None, None]),
        ('L', [Some(Dir::North), Some(Dir::East), None, None]),
        ('7', [Some(Dir::South), Some(Dir::West), None, None]),
        ('F', [Some(Dir::South), Some(Dir::East), None, None]),
        ('J', [Some(Dir::North), Some(Dir::West), None, None]),
        (
            'S',
            [
                Some(Dir::North),
                Some(Dir::South),
                Some(Dir::West),
                Some(Dir::East),
            ],
        ),
        ('.', [None; 4]),
    ]);

    let s = find_element_index(&lines, &'S').unwrap();
    let mut q = VecDeque::new();
    q.push_back(s);
    let mut vis: Vec<Vec<u8>> = vec![vec![0; lines[0].len() + 2]; lines.len() + 2];
    let mut dist: Vec<Vec<u32>> = vec![vec![u32::MAX; lines[0].len() + 2]; lines.len() + 2];
    let mut par: Vec<Vec<(usize, usize)>> = vec![vec![(0, 0); lines[0].len() + 2]; lines.len() + 2];

    dist[s.0][s.1] = 0;
    vis[s.0][s.1] = 1;
    par[s.0][s.1] = s;
    while let Some((i, j)) = q.pop_front() {
        vis[i][j] = 1;
        let temp = &pipes_map[&lines[i - 1][j - 1]];
        for v in temp.iter().flatten().map(|d| {
            return dir2coords(i, j, *d);
        }) {
            // dbg!(v);
            if v.0 == 0 || v.1 == 0 || v.0 > lines.len() || v.1 > lines[0].len() {
                continue;
            }
            let nb_temp = lines[v.0 - 1][v.1 - 1];
            if nb_temp == '.' {
                continue;
            }
            if v == par[i][j] {
                continue;
            }
            if vis[v.0][v.1] == 1 {
                continue;
            }
            if pipes_map[&nb_temp]
                .iter()
                .flatten()
                .map(|d| return dir2coords(v.0, v.1, *d))
                .find(|&x| return x == (i, j))
                .is_some()
            {
                dist[v.0][v.1] = std::cmp::min(dist[i][j] + 1, dist[v.0][v.1]);
                par[v.0][v.1] = (i, j);
                q.push_back(v);
            }
        }
    }

    dist.iter_mut().for_each(|r| {
        r.iter_mut().for_each(|e| {
            if *e == u32::MAX {
                *e = 0;
            }
        })
    });

    let ans = dist.iter().flatten().max().unwrap();
    println!("Part 1: {}", ans);
    return dist;
}
fn part2(dist: Vec<Vec<u32>>) {
    let input = handle_input();
    let mut lines: Vec<Vec<_>> = input.lines().map(|s| s.chars().collect()).collect();
    let s = find_element_index(&lines, &'S').unwrap();

    let dx = [-1, 1, 0, 0];
    let dy = [0, 0, 1, -1];
    let mut dirs = vec![];
    let pipes_map: HashMap<char, [Option<Dir>; 4]> = HashMap::from([
        ('|', [Some(Dir::North), Some(Dir::South), None, None]),
        ('-', [Some(Dir::East), Some(Dir::West), None, None]),
        ('L', [Some(Dir::North), Some(Dir::East), None, None]),
        ('7', [Some(Dir::South), Some(Dir::West), None, None]),
        ('F', [Some(Dir::South), Some(Dir::East), None, None]),
        ('J', [Some(Dir::North), Some(Dir::West), None, None]),
        (
            'S',
            [
                Some(Dir::North),
                Some(Dir::South),
                Some(Dir::East),
                Some(Dir::West),
            ],
        ),
        ('.', [None; 4]),
    ]);
    let mut dir_to_pipes: HashMap<Vec<Option<Dir>>, char> = HashMap::new();

    for (k, v) in pipes_map {
        *dir_to_pipes
            .entry(v.into_iter().collect())
            .or_insert_with(|| k) = k;
    }

    for i in 0..4 {
        let di = s.0 as i32 + dx[i];
        let dj = s.1 as i32 + dy[i];
        if di < 0
            || dj < 0
            || di > lines.len().try_into().unwrap()
            || dj > lines[0].len().try_into().unwrap()
        {
            continue;
        }
        if dist[di as usize][dj as usize] == 1 {
            dirs.push(Some(coords2dir(s.0, s.1, di as usize, dj as usize)));
        }
    }
    while dirs.len() < 4 {
        dirs.push(None);
    }
    // dbg!(&dirs);
    let dir = dir_to_pipes[&dirs];
    // dbg!(dirs,dir);
    lines[s.0 - 1][s.1 - 1] = dir;

    let mut count = vec![vec![0; lines[0].len()]; lines.len()];
    let mut val;
    for i in 0..lines.len() {
        val = 0;
        for j in 0..lines[i].len() {
            // check if part of main loop
            if dist[i + 1][j + 1] != 0 || s == (i + 1, j + 1) {
                if lines[i][j] == '|' || lines[i][j] == '7' || lines[i][j] == 'F' {
                    val += 1;
                }
                continue;
            } else {
                count[i][j] = val;
            }
        }
    }
    let mut ans = 0;
    for r in &count {
        for x in r {
            if x % 2 == 1 {
                ans += 1;
            }
        }
    }
    println!("Part 2: {}", ans);
}

pub fn solve(x: bool) {
    let d = part1();
    if x {
        part2(d);
    }
}
