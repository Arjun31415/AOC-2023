fn handle_input() -> String {
    let input = include_str!("../inputs/day1/day1.in").to_string();
    return input;
}
pub fn part1() {
    println!("PART 1");
    let input = handle_input();
    let mut s = 0;
    for line in input.lines() {
        let a = line
            .chars()
            .find(char::is_ascii_digit)
            .unwrap()
            .to_digit(10)
            .unwrap();
        let b = line
            .chars()
            .rev()
            .find(char::is_ascii_digit)
            .unwrap()
            .to_digit(10)
            .unwrap();
        let num = a * 10 + b;
        s += num;
    }
    println!("{}", s);
}
pub fn part2() {
    println!("PART 2");
    let mut input = handle_input();
    let mut s = 0;
    let replace = [
        ("one", "o1e"), // because if it is oneight-> o1eight -> o1e8t otherwise the 'e' of eight gets eaten up
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];
    for (w, rep) in replace {
        input = input.replace(w, rep);
    }
    for line in input.lines() {
        println!("{}", line);
        let a = line
            .chars()
            .find(char::is_ascii_digit)
            .unwrap()
            .to_digit(10)
            .unwrap();
        let b = line
            .chars()
            .rev()
            .find(char::is_ascii_digit)
            .unwrap()
            .to_digit(10)
            .unwrap();
        let num = a * 10 + b;
        s += num;
    }
    println!("{}", s);
}
pub fn solve(x: bool) {
    if x {
        part2();
    } else {
        part1();
    }
}
