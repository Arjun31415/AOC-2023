use std::{cmp::Ordering, collections::HashMap};

fn handle_input() -> String {
    let binding = include_str!("../inputs/day7/day7.in").to_string();
    let input = binding.trim_end();
    return input.to_string();
}
fn rank_v2(a: &Vec<u32>) -> u32 {
    let f = a.iter().copied().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
        map
    });
    let num_j: u32 = *f.get(&0).unwrap_or(&0);
    let mut max_val = 0;
    let mut max_key =0;
    for (k,v) in &f{
        if *k==0{continue;}
        max_val =std::cmp::max(max_val,*v);
    }
    for (k,v) in &f{
        if *k==0{continue;}
        if *v== max_val {max_key=*k;} 
    }
    let mut b = a.clone();
    b.retain(|v| *v != 0);
    for _ in 0..num_j {
        b.push(max_key);
    }
    if num_j >=1 {
    dbg!(&a,&b);}
    return rank(&b);
}
fn rank(a: &Vec<u32>) -> u32 {
    let f = a.iter().copied().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
        map
    });
    if f.len() == 1 {
        return 7;
    } else if f.len() == 5 {
        return 1;
    } else if f.len() == 4 {
        return 2;
    } else if f.len() == 3 {
        for (_, v) in f {
            // 3 of a kind
            if v == 3 {
                return 4;
            }
        }
        // two pair
        return 3;
    } else if f.len() == 2 {
        for (_, v) in f {
            // four of a kind
            if v == 4 {
                return 6;
            }
        }
        // full house
        return 5;
    }

    return 0;
}
fn part1() {
    let input = handle_input();
    let mut mp: HashMap<char, u32> =
        HashMap::from([('A', 14), ('K', 13), ('Q', 12), ('J', 11), ('T', 10)]);
    mp.extend(('2'..='9').map(|n| (n, n.to_digit(10).unwrap())));

    let temp: Vec<_> = input
        .lines()
        .map(|l| l.split(' ').collect::<Vec<&str>>())
        .collect();
    let mut a: Vec<(Vec<u32>, u32)> = vec![];
    for t in temp {
        let x: Vec<u32> = t[0].chars().map(|c| mp[&c]).collect::<Vec<u32>>();
        a.push((x, t[1].parse().unwrap()));
    }
    a.sort_unstable_by(|x, y| {
        let r1 = rank(&x.0);
        let r2 = rank(&y.0);
        if r1.cmp(&r2) == Ordering::Equal {
            return x.0.cmp(&y.0);
        }
        return r1.cmp(&r2);
    });
    // dbg!(a);
    let mut ans = 0;
    for (i, (_, v)) in a.iter().enumerate() {
        ans += v * (i as u32 + 1);
    }
    println!("{}", ans);
    // dbg!(&temp);
}
fn part2() {
    let input = handle_input();
    let mut mp: HashMap<char, u32> =
        HashMap::from([('A', 14), ('K', 13), ('Q', 12), ('J', 0), ('T', 10)]);
    mp.extend(('2'..='9').map(|n| (n, n.to_digit(10).unwrap())));

    let temp: Vec<_> = input
        .lines()
        .map(|l| l.split(' ').collect::<Vec<&str>>())
        .collect();
    let mut a: Vec<(Vec<u32>, u32)> = vec![];
    for t in temp {
        let x: Vec<u32> = t[0].chars().map(|c| mp[&c]).collect::<Vec<u32>>();
        a.push((x, t[1].parse().unwrap()));
    }
    a.sort_unstable_by(|x, y| {
        let r1 = rank_v2(&x.0);
        let r2 = rank_v2(&y.0);
        if r1.cmp(&r2) == Ordering::Equal {
            return x.0.cmp(&y.0);
        }
        return r1.cmp(&r2);
    });
    // dbg!(&a);
    let mut ans = 0;
    for (i, (_, v)) in a.iter().enumerate() {
        ans += v * (i as u32 + 1);
    }
    println!("{}", ans);
}

pub fn solve(x: bool) {
    if x {
        part2();
    } else {
        part1();
    }
}
