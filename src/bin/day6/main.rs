// day 6

use std::collections::HashSet;

fn main() {
    let contents =
        std::fs::read_to_string("src/bin/day6/input.txt").expect("unable to read input.txt");

    // part 1
    let v: usize = contents
        .trim()
        .split("\n\n")
        .map(|g| g.replace("\n", "").chars().collect::<HashSet<_>>().len())
        .sum();
    println!("{}", v);

    // part 2
    let v: usize = contents
        .trim()
        .split("\n\n")
        .map(|g| {
            g.split("\n")
                .map(|p| p.chars().collect::<HashSet<_>>())
                .fold(
                    "abcdefghijklmnopqrstuvwxyz".chars().collect::<HashSet<_>>(),
                    |acc, x| acc.intersection(&x).cloned().collect(),
                )
                .len()
        })
        .sum();
    println!("{}", v);
}
