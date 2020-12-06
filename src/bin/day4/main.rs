// day 4

use std::collections::HashSet;

fn part1_validate(p: &Vec<Vec<&str>>) -> bool {
    let req: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .cloned()
        .collect();
    let k: HashSet<&str> = p.iter().map(|kv| kv[0]).collect();

    req.difference(&k).count() == 0
}

fn validate_range(v: &str, start: u32, end: u32) -> bool {
    let d: u32 = v.parse().unwrap_or(0);
    d >= start && d <= end
}

fn validate_height(v: &str) -> bool {
    match v.strip_suffix("in") {
        Some(x) => validate_range(x, 59, 76),
        None => match v.strip_suffix("cm") {
            Some(x) => validate_range(x, 150, 193),
            None => false,
        },
    }
}

fn part2_validate(p: &Vec<Vec<&str>>) -> bool {
    p.iter()
        .filter(|kv| {
            let (k, v) = (kv[0], kv[1]);
            match k {
                "byr" => validate_range(v, 1920, 2002),
                "iyr" => validate_range(v, 2010, 2020),
                "eyr" => validate_range(v, 2020, 2030),
                "hgt" => validate_height(v),
                "hcl" => {
                    v.len() == 7
                        && v.chars().nth(0).unwrap() == '#'
                        && v[1..].chars().all(|c| c.is_ascii_hexdigit())
                }
                "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v),
                "pid" => v.len() == 9 && v.chars().all(|c| c.is_numeric()),
                _ => false,
            }
        })
        .count()
        == 7
}

fn main() {
    let contents =
        std::fs::read_to_string("src/bin/day4/input.txt").expect("unable to read input.txt");

    let passports: Vec<Vec<Vec<&str>>> = contents
        .trim()
        .split("\n\n")
        .map(|p| {
            p.split(|c| c == ' ' || c == '\n')
                .map(|f| f.split(':').collect())
                .collect()
        })
        .collect();

    // Part 1
    let c = passports.clone().into_iter().filter(part1_validate).count();
    println!("{}", c);

    // Part 2
    let c = passports.into_iter().filter(part2_validate).count();
    println!("{}", c);
}
