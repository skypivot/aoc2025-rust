use std::fs::read_to_string;

fn main() {
    let ranges: Vec<_> = read_to_string("input/day2.txt")
        .unwrap()
        .split(",")
        .map(|s| s.trim())
        .map(String::from)
        .filter(|s| !s.is_empty())
        .map(|s| {
            let split: Vec<_> = s.splitn(2, "-").collect();
            (split[0].to_owned(), split[1].to_owned())
        })
        .collect();

    let nr_repeats_once = |n: i64| {
        let s = format!("{}", n);
        s[..(s.len() / 2)] == s[(s.len() / 2)..]
    };

    let nr_repeats_any = |n| {
        let s = format!("{}", n);
        for div in 1..(s.len() / 2) + 1 {
            let slice = s.as_bytes();
            if slice.len() % div == 0 {
                let chunks = slice.chunks(div).collect::<Vec<_>>();
                //println!("{} {} Chunks: {:?}", n, div, chunks);
                let mut all_equal = true;
                for i in 1..chunks.len() {
                    if chunks[0] != chunks[i] {
                        all_equal = false
                    }
                }
                if all_equal {
                    return true;
                }
            }
        }
        false
    };

    let mut invalid_sum = 0;
    let mut invalid_sum_new = 0;

    for (start, end) in ranges {
        let start = start.parse::<i64>().expect("invalid");
        let end = end.parse::<i64>().expect("invalid");
        for n in start..=end {
            if nr_repeats_once(n) {
                //println!("{}", n);
                invalid_sum += n;
            }
            if nr_repeats_any(n) {
                //println!("Invalid: {}", n);
                invalid_sum_new += n;
            }
        }
    }

    println!("{}", invalid_sum);
    println!("{}", invalid_sum_new);
}
