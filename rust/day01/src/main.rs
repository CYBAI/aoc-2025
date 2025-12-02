fn main() {
    let input = include_str!("../../../inputs/day01.txt");

    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> i32 {
    let rotations = parse_rotations(input);

    let mut curr_position = 50;
    let mut password = 0;
    for rotation in rotations.iter() {
        curr_position = rotation.move_from(curr_position);
        password += i32::from(curr_position == 0);
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
    fn move_from(&self, position: i32) -> i32 {
        match self.direction {
            Direction::Left => (position - self.distance) % 100,
            Direction::Right => (position + self.distance) % 100,
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

    #[test]
    fn test_part1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(part1(input), 3);
    }
}
