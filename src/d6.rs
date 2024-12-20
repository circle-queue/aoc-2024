use std::{
    collections::HashSet,
    fs,
    io::{stdout, Write},
};

fn main() {
    one();
    two();
}

fn read() -> Vec<Vec<char>> {
    let lines = fs::read_to_string("input/d6.txt").unwrap();
    return lines.lines().map(|x| x.chars().collect()).collect();
}

fn move_(
    input: &Vec<Vec<char>>,
    pos: (i32, i32),
    dir: (i32, i32),
) -> Option<((i32, i32), (i32, i32))> {
    if let (Ok(i_), Ok(j_)) = (
        usize::try_from(pos.0 + dir.0),
        usize::try_from(pos.1 + dir.1),
    ) {
        let new_pos = (i_ as i32, j_ as i32);
        if let Some(row) = (*input).get(i_) {
            if let Some(&cell) = row.get(j_) {
                if cell == '#' {
                    // rotate 90
                    return Some((pos, (dir.1, -dir.0)));
                } else {
                    return Some((new_pos, dir));
                }
            }
        }
    }
    None
}

fn solve_one(input: &Vec<Vec<char>>) -> Option<usize> {
    let mut dir = (-1, 0);

    let mut pos = input
        .iter()
        .enumerate()
        .filter_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find(|(_, &x)| x == '^')
                .and_then(|(j, _)| Some((i as i32, j as i32)))
        })
        .next()
        .unwrap();

    let mut tried = HashSet::from([(pos, dir)]);
    let mut seen = HashSet::from([pos]);
    while let Some((pos_, dir_)) = move_(&input, pos, dir) {
        pos = pos_;
        dir = dir_;
        seen.insert(pos);
        if tried.contains(&(pos, dir)) {
            return None;
        }
        tried.insert((pos, dir));
    }
    return Some(seen.len());
}

fn one() {
    let input = read();
    println!("{}", solve_one(&input).unwrap())
}

fn two() {
    // Brute force ~a minute
    let input = read();
    let mut mut_input = input.clone();
    let mut ans = 0;
    for (i, row) in input.iter().enumerate() {
        for (j, &symbol) in row.iter().enumerate() {
            if symbol == '.' {
                mut_input[i][j] = '#';
                if solve_one(&mut_input).is_none() {
                    ans += 1;
                }
                mut_input[i][j] = '.';
            }
        }
        // Pretty print progress
        print!("{:.0}%", (i * 100) as f32 / input.len() as f32);
        stdout().flush().unwrap();
        print!("\r")
    }
    println!("{}", ans)
}
