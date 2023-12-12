use num::{self, Integer};
use std::collections::HashMap;

fn handle_input() -> String {
    let binding = include_str!("../inputs/day8/day8.in").to_string();
    let input = binding.trim_end();
    return input.to_string();
}
fn part1() {
    let input = handle_input();
    let mut lines = input.lines();
    let inst: Vec<char> = lines.next().unwrap().chars().collect();
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    lines.next();
    for line in lines {
        let temp = line
            .replace("(", "")
            .replace(")", "")
            .replace("=", "")
            .replace(",", "");
        let temp: Vec<&str> = temp.split(" ").filter(|w| !w.is_empty()).collect();
        adj.insert(
            temp[0].to_string(),
            vec![temp[1].to_string(), temp[2].to_string()],
        );
    }
    let mut cur = "AAA";
    let des = "ZZZ";
    let mut i = 0;
    let mut ans = 0;

    while cur != des {
        // dbg!(inst[i],cur,ans);
        if inst[i] == 'R' {
            cur = &adj[cur][1];
        } else {
            cur = &adj[cur][0];
        }

        ans += 1;
        i += 1;
        i = i % inst.len();
    }
    println!("{}", ans);
}
fn part2() {
    let input = handle_input();
    let mut lines = input.lines();
    let inst: Vec<char> = lines.next().unwrap().chars().collect();
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    lines.next();
    let mut nodes: Vec<String> = vec![];
    for line in lines {
        let temp = line
            .replace("(", "")
            .replace(")", "")
            .replace("=", "")
            .replace(",", "");
        let temp: Vec<&str> = temp.split(" ").filter(|w| !w.is_empty()).collect();
        adj.insert(
            temp[0].to_string(),
            vec![temp[1].to_string(), temp[2].to_string()],
        );
        nodes.push(temp[0].to_string());
    }
    let cur: Vec<String> = nodes
        .iter()
        .filter(|w| w.ends_with("A"))
        .map(|w| w.to_owned())
        .collect();
    let dest: Vec<String> = nodes
        .iter()
        .filter(|w| w.ends_with('Z'))
        .map(|w| w.to_owned())
        .collect();
    let mut i: usize;
    let mut ans: u32;
    dbg!(cur.len(), dest.len());
    let mut f: HashMap<String, HashMap<String, Vec<u32>>> = HashMap::new();
    for c in &cur {
        f.insert(c.to_string(), HashMap::new());
    }

    for j in 0..cur.len() {
        let mut u = cur[j].to_owned();
        i = 0;
        ans = 0;
        while ans <= 100000 {
            // dbg!(inst[i],cur,ans);
            if inst[i] == 'R' {
                u = adj[&u][1].to_owned();
            } else {
                u = adj[&u][0].to_owned();
            }
            ans += 1;
            i += 1;
            i = i % inst.len();
            if u.ends_with('Z') {
                f.entry(cur[j].to_owned())
                    .or_insert(HashMap::new())
                    .entry(u.to_owned())
                    .or_insert([].to_vec())
                    .push(ans);
            }
        }
    }
    let mut ans: u64 = 1;
    for (_, l1) in f {
        for (_, x) in l1 {
            ans = ans.lcm(&(x[0] as u64));
        }
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
