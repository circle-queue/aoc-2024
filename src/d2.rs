use std::fs;

fn main() {
    two()
}

fn read() -> Vec<Vec<i32>> {
    let input = fs::read_to_string("input/d2.txt").unwrap();
    input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .into_iter()
                .map(|e| e.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn pairs(row: &[i32]) -> impl Iterator<Item = (&i32, &i32)> {
    row.iter().zip(row.iter().skip(1))
}

fn safe(row: &[i32]) -> bool {
    let increasing = pairs(row).all(|(&l, &r)| l < r);
    let decreasing = pairs(row).all(|(&l, &r)| l > r);
    let small = pairs(row).all(|(&l, &r)| 1 <= l.abs_diff(r) && l.abs_diff(r) <= 3);
    return (increasing || decreasing) && small;
}

fn one() {
    let grid = read();
    let mut ans = 0;
    for row in grid {
        ans += safe(&row) as u32
    }
    println!("{ans}")
}

fn two() {
    let grid = read();
    let mut ans = 0;
    for row in grid {
        for (drop_idx, _) in row.iter().enumerate() {
            let new_row: Vec<i32> = row
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != drop_idx)
                .map(|(_, &x)| x)
                .collect();
            if safe(&new_row) {
                ans += 1;
                break;
            }
        }
    }
    println!("{ans}")
}
