// day 2

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Debug)]
struct Entry {
    p1: i32,
    p2: i32,
    c: char,
    password: String,
}

impl FromStr for Entry {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Splits an input line in the form
        //
        // 3-5 f: fgfff
        //
        // Into a vector of ["3", "5", "f", "", "fgfff"]
        let parts: Vec<&str> = s.split(|c| c == '-' || c == ' ' || c == ':').collect();

        // TODO: I know I am doing this error handling all wrong, but I
        // don't grok how to do it yet.
        Ok(Entry {
            p1: parts[0].parse().unwrap(),
            p2: parts[1].parse().unwrap(),
            c: parts[2].chars().next().unwrap(),
            password: parts[4].to_string(),
        })
    }
}

fn main() {
    let file = File::open("src/day2/input.txt").expect("unable to open input.txt");
    let reader = BufReader::new(file);

    let mut part1_count: i32 = 0;
    let mut part2_count: i32 = 0;
    for l in reader.lines() {
        let e = Entry::from_str(l.unwrap().as_str()).unwrap();
        let c = e.password.chars().filter(|c| *c == e.c).count() as i32;

        // Part 1: treat p1 and p2 as the min and max number of occurrences
        if c >= e.p1 && c <= e.p2 {
            part1_count += 1;
        }

        // Part 2: treat p1 and p2 as 1-indexed positions in the password,
        // check if the character is present in those positions and xor them.
        let first_ok = e.password.chars().nth((e.p1 - 1) as usize).unwrap() == e.c;
        let second_ok = e.password.chars().nth((e.p2 - 1) as usize).unwrap() == e.c;
        if first_ok ^ second_ok {
            part2_count += 1;
        }
    }

    println!("{}", part1_count);
    println!("{}", part2_count);
}
