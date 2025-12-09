use std::fs::read_to_string;

use itertools::Itertools;
use itertools::enumerate;

fn main() {
    let banks: Vec<_> = read_to_string("input/day3.txt")
        .unwrap()
        .lines()
        .map(|s| s.trim())
        .map(String::from)
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect_vec()
        })
        .collect();

    let max_tup = |slice: &[i64]| {
        let (mut i_max, mut max) = (0, 0);
        for (i, v) in enumerate(&slice[0..]) {
            if *v > max {
                i_max = i;
                max = *v;
            }
        }
        (i_max, max)
    };
    let max_tup_n = |n: usize, slice: &[i64]| {
        let (mut max_digit_i, v) = max_tup(&slice[..(slice.len() - n + 1)]);
        let mut total = 10_i64.pow((n - 1) as u32) * v;
        for digit in 1..n {
            let reserved_digits = n - digit - 1;
            let (i, v) = max_tup(&slice[(max_digit_i + 1)..(slice.len() - reserved_digits)]);
            max_digit_i += i + 1;
            total += 10_i64.pow((n - (digit + 1)) as u32) * v;
        }
        total
    };
    let mut sum_1 = 0;
    let mut sum_2 = 0;
    for bank in banks {
        sum_1 += max_tup_n(2, &bank);
        sum_2 += max_tup_n(12, &bank);
    }
    println!("{}", sum_1);
    println!("{}", sum_2);
}
