// day 5

fn seat(l: &str) -> u32 {
    let mut seat_id = 0;

    for x in l.chars() {
        match x {
            'B' | 'R' => seat_id = seat_id << 1 | 1,
            _ => seat_id = seat_id << 1,
        }
    }
    seat_id
}

fn main() {
    let contents =
        std::fs::read_to_string("src/bin/day5/input.txt").expect("unable to read input.txt");

    let seat_ids: Vec<u32> = contents.lines().map(|l| seat(&l)).collect();

    // part 1
    let max = seat_ids.iter().max().unwrap();
    println!("{}", max);

    // part 2
    let mut sorted = seat_ids.clone();
    sorted.sort();

    for (i, s) in sorted.iter().enumerate() {
        if sorted[i + 1] != s + 1 {
            println!("{}", s + 1);
            break;
        }
    }
}
