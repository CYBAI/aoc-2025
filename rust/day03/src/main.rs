fn main() {
    let input = include_str!("../../../inputs/day03.txt");

    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> u64 {
    input
        .trim()
        .lines()
        .map(|line| {
            let nums = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<u64>>();

            find_largest_joltage(&nums)
        })
        .sum()
}

fn find_largest_joltage(batteries: &[u64]) -> u64 {
    let len = batteries.len();

    let Some((max_index, ten)) = find_max(batteries, 0..(len - 1)) else {
        return 0;
    };

    let Some((_, unit)) = find_max(batteries, ((max_index + 1)..len).rev()) else {
        return 0;
    };

    ten * 10 + unit
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
        assert_eq!(part1(INPUT), 357);
    }
}
