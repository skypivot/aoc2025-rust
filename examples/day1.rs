use std::fs::read_to_string;

fn main() {
    let instructions: Vec<_> = read_to_string("input/day1.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .filter(|s| !s.is_empty())
        .map(|s| {
            (
                s[..1].to_owned(),
                s[1..].parse::<i64>().expect("non-number in input"),
            )
        })
        .collect();

    //Part 1 - modulo arithmetic will do.
    let mut dial = 50;
    let mut zero_hits = 0;
    for (lr, n) in &instructions {
        if lr == "L" {
            dial = (dial - n) % 100
        } else {
            dial = (dial + n) % 100
        }
        if dial < 0 {
            dial += 100
        }
        if dial == 0 {
            zero_hits += 1
        }
    }
    println!("{}", zero_hits);

    //Part 2 - when do we cross this dial exactly?
    dial = 50;
    let mut zero_passes = 0;
    for (lr, n) in &instructions {
        zero_passes += n / 100;
        let n = n % 100;
        if lr == "L" {
            if dial != 0 && dial - n < 0 {
                zero_passes += 1;
                println!("Zero pass on {}{}", lr, n);
            }
            dial = (dial - n) % 100
        } else {
            if dial + n > 100 {
                zero_passes += 1;
                println!("Zero pass on {}{}", lr, n);
            }
            dial = (dial + n) % 100
        }
        if dial < 0 {
            dial += 100 + (n / 100) * 100
        }
    }
    println!("{}", zero_passes + zero_hits);
}
