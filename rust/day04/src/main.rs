fn main() {
    let input = include_str!("../../../inputs/day04.txt");
    let mut grid = parse_grid(input);

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&mut grid));
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn part1(grid: &[Vec<char>]) -> u32 {
    find_accessible_positions(grid).len() as u32
}

fn part2(grid: &mut [Vec<char>]) -> u32 {
    let mut count = 0;

    loop {
        let pos = find_accessible_positions(grid);
        if pos.is_empty() {
            break;
        }

        count += pos.len() as u32;

        for (r, c) in pos.iter() {
            grid[*r][*c] = '.';
        }
    }

    count
}

fn find_accessible_positions(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let m = grid.len();
    let n = grid[0].len();
    let mut positions = Vec::new();

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '.' {
                continue;
            }

            if is_forklift_accessible(grid, (i, j)) {
                positions.push((i, j));
            }
        }
    }

    positions
}

fn is_forklift_accessible(grid: &[Vec<char>], curr_coord: (usize, usize)) -> bool {
    let (row, col) = curr_coord;
    let m = grid.len();
    let n = grid[0].len();
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (1, -1),
        (1, 1),
        (1, 0),
        (0, -1),
        (0, 1),
    ];

    let mut count = 0;

    for (dr, dc) in directions.iter() {
        let r = row as isize + dr;
        let c = col as isize + dc;

        if r >= 0 && r < m as isize && c >= 0 && c < n as isize {
            if grid[r as usize][c as usize] == '@' {
                count += 1;
            }
        }
    }

    count < 4
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        let grid = parse_grid(INPUT);
        assert_eq!(part1(&grid), 13);
    }

    #[test]
    fn test_part2() {
        let mut grid = parse_grid(INPUT);
        assert_eq!(part2(&mut grid), 43);
    }
}
