// day 1

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("src/bin/day1/input.txt").expect("unable to open input.txt");
    let reader = BufReader::new(file);

    let vals: Vec<i32> = reader
        .lines()
        .map(|l| l.unwrap().parse().expect("expected an integer"))
        .collect();

    'outer1: for x in &vals {
        for y in &vals {
            if x + y == 2020 {
                println!("{}", x * y);
                break 'outer1;
            }
        }
    }

    'outer2: for x in &vals {
        for y in &vals {
            for z in &vals {
                if x + y + z == 2020 {
                    println!("{}", x * y * z);
                    break 'outer2;
                }
            }
        }
    }
}
