use std::{collections::HashMap, fs};

fn main() {
    two()
}

fn read() -> (Vec<i32>, Vec<i32>) {
    let input = fs::read_to_string("input/d1.txt").unwrap();
    input
        .lines()
        .map(|x| {
            let (a, b) = x.split_once("   ").unwrap();
            (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
        })
        .unzip()
}

fn one() {
    let (mut l, mut r) = read();
    l.sort();
    r.sort();
    let ans: u32 = l.iter().zip(r).map(|(l, r)| r.abs_diff(*l)).sum();
    println!("{ans}")
}

fn two() {
    let (l, r) = read();

    let mut r_count: HashMap<i32, i32> = HashMap::new();
    for v in r {
        let entry = r_count.entry(v).or_insert(0);
        *entry += 1;
    }
    let ans: i32 = l
        .iter()
        .map(|&x| x * r_count.get(&x).copied().unwrap_or(0))
        .sum();
    println!("{ans}")
}
