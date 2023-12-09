use num::{self, Integer};
use std::collections::HashMap;

fn handle_input() -> String {
    let binding = include_str!("../inputs/day9/day9.in").to_string();
    let input = binding.trim_end();
    return input.to_string();
}
fn is_all_same<T: PartialEq>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] == w[1])
}
fn part1() {
    let input = handle_input();
    let mut lines = input.lines();
    let mut ans = 0;
    for line in lines {
        let a: Vec<i32> = line
            .split(' ')
            .filter(|q| !q.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let mut diff: Vec<Vec<i32>> = vec![];
        diff.push(a.clone());
        let mut j = 1;
        while true {
            diff.push(Vec::new());
            for i in 0..diff[j - 1].len() - 1 {
                let x = diff[j - 1][i + 1];
                let y = diff[j - 1][i];
                diff[j].push(x - y);
            }
            if (is_all_same(&diff[j])) {
                break;
            }
            j += 1;
        }
        let temp = *diff[j].last().unwrap();
        diff[j].push(temp);
        j -= 1;
        dbg!(j);
        for k in (0..=j).rev() {
            let x = *diff[k].last().unwrap();
            let y = *diff[k + 1].last().unwrap();
            diff[k].push(x + y);
            if(k==0){ans+=(x+y);}

        }
        // dbg!(diff);
    }
    println!("{}",ans);
}
fn part2() {
    let input = handle_input();
    let mut lines = input.lines();
    let mut ans = 0;
    for line in lines {
        let a: Vec<i32> = line
            .split(' ')
            .filter(|q| !q.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let mut diff: Vec<Vec<i32>> = vec![];
        diff.push(a.clone());
        diff[0].reverse();
        let mut j = 1;
        while true {
            diff.push(Vec::new());
            for i in 0..diff[j - 1].len() - 1 {
                let x = diff[j - 1][i + 1];
                let y = diff[j - 1][i];
                diff[j].push(x - y);
            }
            if (is_all_same(&diff[j])) {
                break;
            }
            j += 1;
        }
        let temp = *diff[j].last().unwrap();
        diff[j].push(temp);
        j -= 1;
        dbg!(j);
        for k in (0..=j).rev() {
            let x = *diff[k].last().unwrap();
            let y = *diff[k + 1].last().unwrap();
            diff[k].push(x + y);
            if(k==0){ans+=(x+y);}

        }
        // dbg!(diff);
    }
    println!("{}",ans);
}

pub fn solve(x: bool) {
    if x {
        part2();
    } else {
        part1();
    }
}
