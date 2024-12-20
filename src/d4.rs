use std::fs;

fn main() {
    one();
    two();
}

fn read() -> Vec<Vec<char>> {
    fs::read_to_string("input/d4.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn count_xmas(input: &Vec<Vec<char>>, i: i32, j: i32) -> i32 {
    let mut total = 0;
    let empty: Vec<char> = Vec::new();
    for di in -1..2 {
        for dj in -1..2 {
            let (mut i_, mut j_) = (i, j);
            let mut found = 0;
            for char_ in "XMAS".chars() {
                if let (Ok(i__), Ok(j__)) = (usize::try_from(i_), usize::try_from(j_)) {
                    if let Some(char__) = input.get(i__).unwrap_or(&empty).get(j__) {
                        found += (char__ == &char_) as i32;
                    };
                };
                i_ += di;
                j_ += dj;
            }
            if found == 4 {
                total += 1
            }
        }
    }
    return total;
}

fn one() {
    let input = read();
    let mut ans = 0;
    for (i, line_) in input.iter().enumerate() {
        for (j, _) in line_.iter().enumerate() {
            ans += count_xmas(&input, i as i32, j as i32)
        }
    }
    println!("{ans}")
}

#[rustfmt::skip]
fn count_x_mas(input: &Vec<Vec<char>>, i: i32, j: i32) -> i32 {
    if let (Ok(i_), Ok(j_)) = (usize::try_from(i - 1), usize::try_from(j - 1)) {
        let x = input
            .get(i_..i_ + 3)
            .unwrap_or_default()
            .iter()
            .map(|row| row.get(j_..j_ + 3).unwrap_or_default())
            .flatten()
            .collect::<Vec<_>>();

        match x[..] {
            ['M', _, 'M', 
            _, 'A', _, 
            'S', _, 'S'] => 1,

            ['M', _, 'S', 
            _, 'A', _, 
            'M', _, 'S'] => 1,

            ['S', _, 'S', 
            _, 'A', _, 
            'M', _, 'M'] => 1,

            ['S', _, 'M', 
            _, 'A', _, 
            'S', _, 'M'] => 1,

            _ => 0,
        }
    } else {
        0
    }
}

fn two() {
    let input = read();
    let mut ans = 0;
    for (i, line_) in input.iter().enumerate() {
        for (j, _) in line_.iter().enumerate() {
            ans += count_x_mas(&input, i as i32, j as i32)
        }
    }
    println!("{ans}")
}
