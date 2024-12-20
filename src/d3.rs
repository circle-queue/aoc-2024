use regex::Regex;
use std::fs;

fn main() {
    one();
    two();
}

fn read() -> String {
    fs::read_to_string("input/d3.txt").unwrap()
}

fn one() {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let input = read();
    let mut ans = 0;
    for (_, [a, b]) in re.captures_iter(&input).map(|c| c.extract()) {
        ans += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
    }
    println!("{ans}")
}

fn two() {
    let re = Regex::new(r"(do|don't)(\(\))|mul\((\d+),(\d+)\)").unwrap();
    let input = read();
    let mut ans = 0;
    let mut do_ = true;
    for (_, [a, b]) in re.captures_iter(&input).map(|c| c.extract()) {
        match a {
            "do" => do_ = true,
            "don't" => do_ = false,
            _ if do_ => ans += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap(),
            _ => (),
        };
    }
    println!("{ans}")
}
