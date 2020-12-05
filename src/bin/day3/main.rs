// day 3

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Space {
    Open,
    Tree,
}

#[derive(Debug)]
struct Forest {
    spaces: Vec<Vec<Space>>,
}

impl Forest {
    fn new(r: &mut dyn BufRead) -> Forest {
        let spaces = r
            .lines()
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| match c {
                        '.' => Space::Open,
                        '#' => Space::Tree,
                        _ => panic!("unknown space"),
                    })
                    .collect()
            })
            .collect();
        Forest { spaces }
    }

    fn toboggan(&self, slope: &Slope) -> Vec<Space> {
        let mut row = 0;
        let mut col = 0;
        let height = self.spaces.len();
        let width = self.spaces[0].len();
        let mut v: Vec<Space> = Vec::new();

        while row < height {
            v.push(self.spaces[row][col]);
            row += slope.y;
            col = (col + slope.x) % width;
        }
        v
    }
}

#[derive(Debug)]
struct Slope {
    x: usize,
    y: usize,
}

fn main() {
    let file = File::open("src/bin/day3/input.txt").expect("unable to open input.txt");
    let mut reader = BufReader::new(file);

    let forest = Forest::new(&mut reader);

    // Part 1
    let slope = Slope { x: 3, y: 1 };
    println!(
        "{}",
        forest
            .toboggan(&slope)
            .iter()
            .filter(|s| **s == Space::Tree)
            .count()
    );

    // Part 2
    let slopes = vec![
        Slope { x: 1, y: 1 },
        Slope { x: 3, y: 1 },
        Slope { x: 5, y: 1 },
        Slope { x: 7, y: 1 },
        Slope { x: 1, y: 2 },
    ];

    let p: usize = slopes
        .iter()
        .map(|sl| {
            forest
                .toboggan(&sl)
                .iter()
                .filter(|s| **s == Space::Tree)
                .count()
        })
        .product();

    println!("{}", p);
}
