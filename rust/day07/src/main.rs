use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../../inputs/day07.txt");
    let teleporter = parse(input);

    println!("Part 1: {}", part1(&teleporter));
    println!("Part 2: {}", part2(&teleporter));
}

type Coord = (usize, usize);

struct Teleporter {
    start: Coord,
    /// HashMap<y-coordinate, HashSet<x-coordinates>>
    splitters: HashMap<usize, HashSet<usize>>,
    max_y: usize,
}

fn parse(input: &str) -> Teleporter {
    let mut splitters: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut start: Coord = (0, 0);
    let mut max_y = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start = (x, y);
            }

            if ch == '^' {
                splitters.entry(y).or_default().insert(x);
                max_y = max_y.max(y);
            }
        }
    }

    Teleporter {
        start,
        splitters,
        max_y,
    }
}

fn part1(teleporter: &Teleporter) -> usize {
    let mut count = 0;

    let (curr_x, mut curr_y) = teleporter.start;
    // Move down from starting point
    curr_y += 1;

    let mut curr_rows = HashSet::new();
    curr_rows.insert(curr_x);

    while curr_y <= teleporter.max_y {
        if let Some(splitter_xs) = teleporter.splitters.get(&curr_y) {
            let mut found = 0;
            let mut next_rows = HashSet::new();

            for &x in &curr_rows {
                if splitter_xs.contains(&x) {
                    // Splitter found, create two new paths
                    if x > 0 {
                        next_rows.insert(x - 1);
                    }
                    next_rows.insert(x + 1);
                    found += 1;
                } else {
                    // No splitter, continue straight
                    next_rows.insert(x);
                }
            }

            curr_rows = next_rows;
            count += found;
        }

        curr_y += 1;
    }

    count
}

fn part2(teleporter: &Teleporter) -> usize {
    let (curr_x, curr_y) = teleporter.start;

    fn dfs(x: usize, y: usize, teleporter: &Teleporter, memo: &mut HashMap<Coord, usize>) -> usize {
        if y > teleporter.max_y {
            return 1;
        }

        if let Some(&cached) = memo.get(&(x, y)) {
            return cached;
        }

        let mut total_paths = 0;

        if let Some(splitter_xs) = teleporter.splitters.get(&y) {
            if splitter_xs.contains(&x) {
                // Splitter found, explore both paths
                if x > 0 {
                    total_paths += dfs(x - 1, y + 1, teleporter, memo);
                }
                total_paths += dfs(x + 1, y + 1, teleporter, memo);
            } else {
                // No splitter, continue straight
                total_paths += dfs(x, y + 1, teleporter, memo);
            }
        } else {
            // No splitter in this row, continue straight
            total_paths += dfs(x, y + 1, teleporter, memo);
        }

        memo.insert((x, y), total_paths);
        total_paths
    }

    dfs(curr_x, curr_y + 1, teleporter, &mut HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        let teleporter = parse(INPUT);
        assert_eq!(part1(&teleporter), 21);
    }

    #[test]
    fn test_part2() {
        let teleporter = parse(INPUT);
        assert_eq!(part2(&teleporter), 40);
    }
}
