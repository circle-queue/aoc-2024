use std::{
    collections::{HashMap, HashSet, LinkedList},
    fs,
};

fn main() {
    one();
    two();
}

fn read() -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let lines = fs::read_to_string("input/d5.txt").unwrap();
    let (a, b) = lines.split_once("\n\n").unwrap();

    let pairs = a
        .lines()
        .map(|x| {
            let (x, y) = x.split_once('|').unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    let updates = b
        .lines()
        .map(|x| {
            x.split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    return (pairs, updates);
}

fn valid_update(update: &Vec<i32>, pairs: &Vec<(i32, i32)>) -> bool {
    let mut seen = HashSet::new();
    'outer: for now in update {
        for (first, after) in pairs.iter() {
            if now == first && seen.contains(after) {
                break 'outer;
            }
        }
        seen.insert(now);
    }
    return seen.len() == update.len();
}

fn one() {
    let mut ans = 0;
    let (pairs, updates) = read();
    for update in updates.iter() {
        if valid_update(update, &pairs) {
            ans += update[update.len() / 2];
        }
    }
    println!("{ans}")
}

fn get_order(pairs: &Vec<(i32, i32)>) -> Vec<i32> {
    // Builds a DAG, where the size relates to the #parents
    // Ordering by #parents gives us the correct order
    let mut links = HashMap::<i32, LinkedList<i32>>::new();
    let mut sizes = HashMap::<i32, i32>::new();
    for (first, after) in pairs.iter() {
        let mut seen = HashSet::<i32>::new();
        links.entry(*first).or_default().push_back(*after);

        let size = *sizes.entry(*first).or_insert(1);

        let mut todo = LinkedList::from([*after]);
        while let Some(node) = todo.pop_back() {
            if seen.contains(&node) {
                continue;
            }
            seen.insert(node);
            *sizes.entry(node).or_insert(1) += size;
            if let Some(neigs) = links.get(&node) {
                todo.append(&mut neigs.clone());
            }
        }
    }
    let mut order = Vec::from_iter(sizes);
    order.sort_by(|(_, size1), (_, size2)| size1.cmp(&size2));
    return order.iter().map(|(k, _)| k.to_owned()).collect::<Vec<_>>();
}

fn two() {
    let mut ans = 0;
    let (pairs, updates) = read();
    for update in updates.iter() {
        if !valid_update(update, &pairs) {
            let seen: HashSet<i32> = HashSet::from_iter(update.iter().map(|x| *x));
            // The relevant_pairs MUST form a DAG (all pairs may have cycles, and cannot be ordered)
            let relevant_pairs = pairs
                .iter()
                .filter(|(a, b)| seen.contains(a) && seen.contains(b))
                .map(|x| x.to_owned())
                .collect::<Vec<(i32, i32)>>();
            let ordered = get_order(&relevant_pairs);
            ans += ordered[update.len() / 2];
        }
    }
    println!("{ans}")
}
