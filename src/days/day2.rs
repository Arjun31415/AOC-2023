use std::collections::HashMap;

fn handle_input() -> String {
    let binding = include_str!("../inputs/day2/day2.in").to_string();
    let input = binding.trim_end();
    return input.to_string();
}

fn part1() {
    println!("PART1");
    let mut input = handle_input();
    let replace = [("blue", "b"), ("red", "r"), ("green", "g")];
    for (w, r) in replace {
        input = input.replace(w, r);
    }
    input = input.replace("Game", "").replace(" ", "");
    let mut s = 0;
    for mut line in input.lines() {
        let mut color_count: HashMap<char, i32> = HashMap::new();
        color_count.insert('b', 0);
        color_count.insert('g', 0);
        color_count.insert('r', 0);
        // println!("{:#?}", line);
        let colon_pos = line.find(":").unwrap();
        let id: u32 = line[..colon_pos].to_string().parse().unwrap();
        line = &line[colon_pos + 1..];
        let mut semi_colon_pos: Vec<_> = line.match_indices(";").map(|(i, _)| i).collect();
        semi_colon_pos.push(line.len());

        let mut prev_pos = 0;
        for pos in semi_colon_pos {
            let show = &line[prev_pos..pos];
            println!("{}", show);
            let colors_shown: Vec<_> = show.split(",").collect();
            for color_shown in &colors_shown {
                let color = color_shown.chars().nth(color_shown.len() - 1).unwrap();
                let amount: i32 = color_shown[..color_shown.len() - 1].parse().unwrap();
                println!("{}:{}", color, amount);
                let cur_val = color_count[&color];
                *color_count.entry(color).or_insert(0) = std::cmp::max(cur_val, amount);
            }
            prev_pos = pos + 1;
        }
        dbg!(&color_count);
        if color_count[&'r'] <= 12 && color_count[&'g'] <= 13 && color_count[&'b'] <= 14 {
            s += id;
        }
        println!("{}", s);
    }
}
fn part2() {
    println!("PART1");
    let mut input = handle_input();
    let replace = [("blue", "b"), ("red", "r"), ("green", "g")];
    for (w, r) in replace {
        input = input.replace(w, r);
    }
    input = input.replace("Game", "").replace(" ", "");
    let mut s = 0;
    for mut line in input.lines() {
        let mut color_count: HashMap<char, i32> = HashMap::new();
        color_count.insert('b', 0);
        color_count.insert('g', 0);
        color_count.insert('r', 0);
        // println!("{:#?}", line);
        let colon_pos = line.find(":").unwrap();
        let id: u32 = line[..colon_pos].to_string().parse().unwrap();
        line = &line[colon_pos + 1..];
        let mut semi_colon_pos: Vec<_> = line.match_indices(";").map(|(i, _)| i).collect();
        semi_colon_pos.push(line.len());

        let mut prev_pos = 0;
        for pos in semi_colon_pos {
            let show = &line[prev_pos..pos];
            println!("{}", show);
            let colors_shown: Vec<_> = show.split(",").collect();
            for color_shown in &colors_shown {
                let color = color_shown.chars().nth(color_shown.len() - 1).unwrap();
                let amount: i32 = color_shown[..color_shown.len() - 1].parse().unwrap();
                println!("{}:{}", color, amount);
                let cur_val = color_count[&color];
                *color_count.entry(color).or_insert(0) = std::cmp::max(cur_val, amount);
            }
            prev_pos = pos + 1;
        }
        dbg!(&color_count);
        s += color_count[&'r'] * color_count[&'g'] * color_count[&'b'];
        println!("{}", s);
    }
}

pub fn solve(x: bool) {
    if x {
        part2();
    } else {
        part1();
    }
}
