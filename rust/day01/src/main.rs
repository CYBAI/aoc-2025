fn main() {
    let input = include_str!("../../../inputs/day01.txt");
    let rotations = parse_rotations(input);

    println!("Part 1: {}", part1(&rotations));
    println!("Part 2: {}", part2(&rotations));
}

fn part1(rotations: &[Rotation]) -> i32 {
    let mut curr_position = 50;
    let mut password = 0;

    for rotation in rotations.iter() {
        (curr_position, _) = rotation.move_from(curr_position);
        password += i32::from(curr_position == 0);
    }

    password
}

fn part2(rotations: &[Rotation]) -> i32 {
    let mut curr_position = 50;
    let mut password = 0;

    for rotation in rotations.iter() {
        let (next_position, count) = rotation.move_from(curr_position);
        curr_position = next_position;
        password += count;
    }

    password
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    distance: i32,
}

impl Rotation {
    fn move_from(&self, position: i32) -> (i32, i32) {
        match self.direction {
            Direction::Left => {
                let reversed = (100 - position) % 100;

                (
                    (position - self.distance).rem_euclid(100),
                    (reversed + self.distance) / 100,
                )
            }
            Direction::Right => (
                (position + self.distance) % 100,
                (position + self.distance) / 100,
            ),
        }
    }
}

fn parse_rotations(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .map(|line| {
            let (dir_char, dist_str) = line.split_at(1);
            let direction = match dir_char {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Unsupported direction"),
            };
            let distance = dist_str.parse::<i32>().expect("Invalid distance");
            Rotation {
                direction,
                distance,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        let rotations = parse_rotations(INPUT);
        assert_eq!(part1(&rotations), 3);
    }

    #[test]
    fn test_part2() {
        let rotations = parse_rotations(INPUT);
        assert_eq!(part2(&rotations), 6);
    }
}
