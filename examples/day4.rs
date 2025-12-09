use std::fs::read_to_string;

fn main() {
    let mut grid = read_to_string("input/day4.txt")
        .unwrap()
        .lines()
        .map(|s| s.trim())
        .map(String::from)
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let char_at_grid =
        |grid: &Vec<Vec<char>>, r, c| grid.get(r).and_then(|row: &Vec<char>| row.get(c)).cloned();

    let at_adjacent = |grid: &Vec<Vec<char>>, r, c| {
        //println!("Probing {r}, {c}");
        let mut hits = 0;
        for dr in [-1, 0, 1] {
            for dc in [-1, 0, 1] {
                if !(dr == 0 && dc == 0) {
                    let c = char_at_grid(grid, (r + dr) as usize, (c + dc) as usize);
                    if c == Some('@') {
                        hits += 1;
                    }
                }
            }
        }
        //println!("Hits: {hits}");
        hits
    };

    let mut hits = 0;
    for i in 0..grid.len() as i32 {
        for j in 0..grid[i as usize].len() as i32 {
            let count = at_adjacent(&grid, i, j);
            if grid[i as usize][j as usize] == '@' && count <= 3 {
                hits += 1;
                //println!("Hit at {i}, {j}, count {count}, char {}", grid[i as usize][j as usize]);
            }
        }
    }
    println!("{}", hits);

    let mut removals = 0;
    let mut removed_or_first_run = true;
    while removed_or_first_run {
        removed_or_first_run = false;
        for i in 0..grid.len() as i32 {
            for j in 0..grid[i as usize].len() as i32 {
                let count = at_adjacent(&grid, i, j);
                if grid[i as usize][j as usize] == '@' && count <= 3 {
                    removals += 1;
                    removed_or_first_run = true;
                    grid[i as usize][j as usize] = 'x';
                }
            }
        }
    }
    println!("{}", removals);
}
