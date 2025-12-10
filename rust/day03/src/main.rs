fn main() {
    let input = include_str!("../../../inputs/day03.txt");
    let banks = parse(input);

    println!("Part 1: {}", part1(&banks));
    println!("Part 2: {}", part2(&banks));
}

fn parse(input: &str) -> Vec<Vec<u64>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>()
}

fn part1(input: &[Vec<u64>]) -> u64 {
    input.iter().map(|nums| find_largest_joltage(nums, 2)).sum()
}

fn part2(input: &[Vec<u64>]) -> u64 {
    input
        .iter()
        .map(|nums| find_largest_joltage(nums, 12))
        .sum()
}

fn find_largest_joltage(batteries: &[u64], digits: usize) -> u64 {
    let len = batteries.len();

    if digits == 0 || len < digits {
        return 0;
    }

    let mut start = 0;
    let mut chosen: Vec<(usize, u64)> = Vec::new();

    for i in 0..digits {
        let end_inclusive = len - (digits - i);

        let Some((idx, val)) = find_max(batteries, start..=end_inclusive) else {
            return 0;
        };

        chosen.push((idx, val));
        start = idx + 1;
    }

    chosen.iter().fold(0, |acc, (_, n)| acc * 10 + n)
}

fn find_max(nums: &[u64], range: impl Iterator<Item = usize>) -> Option<(usize, u64)> {
    let mut curr_max: Option<(usize, u64)> = None;
    for i in range {
        let n = nums[i];

        match curr_max {
            None => {
                curr_max = Some((i, n));
            }
            Some((_, max)) if n > max => {
                curr_max = Some((i, n));
            }
            Some(_) => continue,
        }
    }

    curr_max
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        let banks = parse(INPUT);
        assert_eq!(part1(&banks), 357);
    }

    #[test]
    fn test_part2() {
        let banks = parse(INPUT);
        assert_eq!(part2(&banks), 3121910778619);
    }
}
