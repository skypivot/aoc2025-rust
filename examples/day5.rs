use std::{cmp, collections::VecDeque, fs::read_to_string};

fn main() {
    let mut ranges: Vec<(i64, i64)> = vec![];
    let mut items: Vec<i64> = vec![];
    read_to_string("input/day5.txt")
        .unwrap()
        .lines()
        .map(|s| s.trim())
        .map(String::from)
        .filter(|s| !s.is_empty())
        .for_each(|s| {
            if s.find('-').is_some() {
                let (s1, s2) = s.split_once('-').unwrap();
                ranges.push((s1.parse().unwrap(), s2.parse().unwrap()));
            } else if !s.is_empty() {
                items.push(s.parse().unwrap());
            }
        });

    let mut fresh = 0;
    'items: for i in items {
        for (start, end) in &ranges {
            if i >= *start && i <= *end {
                fresh += 1;
                continue 'items;
            }
        }
    }
    println!("{fresh}");

    ranges.sort();
    let mut range_queue: VecDeque<(i64, i64)> = ranges.into_iter().collect();
    let mut current_range = None;
    let mut total_fresh = 0;
    while let Some((start, end)) = range_queue.pop_front() {
        if current_range.is_none() {
            current_range = Some((start, end));
        } else {
            let (old_start, old_end) = current_range.unwrap();
            if start > old_end {
                total_fresh += old_end - old_start + 1;
                current_range = Some((start, end));
            } else {
                current_range = Some((cmp::min(old_start, start), cmp::max(old_end, end)));
            }
        }
        if range_queue.is_empty() {
            let (old_start, old_end) = current_range.unwrap();
            total_fresh += old_end - old_start + 1;
        }
    }

    println!("{total_fresh}");
}
