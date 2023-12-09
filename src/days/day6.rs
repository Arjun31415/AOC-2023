fn handle_input() -> String {
    let binding = include_str!("../inputs/day6/day6.in").to_string();
    let input = binding.trim_end();
    return input.to_string();
}
fn part1() {
    let mut ans = 1;
    let input = handle_input();
    let w = input
        .lines()
        .map(|f| {
            f.split(' ')
                .filter_map(|w| w.parse::<u64>().ok())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    for i in 0..w[0].len() {
        let t = w[0][i];
        let d = w[1][i];
        let (mut x1, mut x2) = solve_quad(1, -(t as i64), d as i64);
        x1 = x1.max(0);
        x2 = x2.min(t as i64);
        let s = (x2 - x1 + 1);
        dbg!(x1, x2, s);
        ans *= s;
    }
    dbg!(ans);
}
fn solve_quad(a: i64, b: i64, c: i64) -> (i64, i64) {
    let x1 = (-b as f64 - ((b * b - 4 * a * c) as f64).sqrt()) / ((2 * a) as f64);
    let x2 = (-b as f64 + ((b * b - 4 * a * c) as f64).sqrt()) / ((2 * a) as f64);
    return ((x1 + 0.1).ceil() as i64, (x2 - 0.1).floor() as i64);
}
fn part2() {
    let mut ans = 1;
    let input = handle_input();
    let w = input
        .lines()
        .map(|f| {
            f.split(' ')
                .filter_map(|w| w.parse::<u64>().ok())
                .fold(0, |acc, elem| {
                    (acc.to_string() + &elem.to_string()).parse().unwrap()
                })
        })
        .collect::<Vec<u64>>();
    dbg!(&w);
    let t = w[0];
    let d = w[1];
    let (mut x1, mut x2) = solve_quad(1, -(t as i64), d as i64);
    x1 = x1.max(0);
    x2 = x2.min(t as i64);
    let s = (x2 - x1 + 1);
    dbg!(t, d, x1, x2, s);

    // for i in 0..w[0].len() {
    //     let t = w[0][i];
    //     let d = w[1][i];
    //     let (mut x1, mut x2) = solve_quad(1, -(t as i64), d as i64);
    //     x1 = x1.max(0);
    //     x2 = x2.min(t as i64);
    //     let s = (x2-x1+1);
    //     dbg!(x1,x2,s);
    //     ans*=s;
    // }
    // dbg!(ans);
}
pub fn solve(x: bool) {
    if x {
        part2();
    } else {
        part1();
    }
}
