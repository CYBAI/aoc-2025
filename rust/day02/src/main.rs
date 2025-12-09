fn main() {
    let input = include_str!("../../../inputs/day02.txt");
    let ranges = parse(input);

    println!("Part 1: {}", part1(&ranges));
}

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn contains(&self, num: u64) -> bool {
        num >= self.start && num <= self.end
    }
}

fn parse(input: &str) -> Vec<Range> {
    input
        .trim()
        .split(',')
        .map(|ranges| {
            let mut parts = ranges.split('-');
            let start = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();
            Range { start, end }
        })
        .collect::<Vec<Range>>()
}

fn part1(ranges: &[Range]) -> u64 {
    let mut ids = vec![];

    for range in ranges.iter() {
        let start_digit = get_digits(range.start);
        let start_half = (start_digit / 2) as u32;
        let start = range.start - range.start % 10u64.pow(start_half);

        for n in start..=range.end {
            if is_invalid_id(n) && range.contains(n) {
                ids.push(n);
            }
        }
    }

    ids.iter().sum()
}

fn get_digits(num: u64) -> u64 {
    (num as f64).log10().floor() as u64 + 1
}

fn is_invalid_id(num: u64) -> bool {
    let digit = get_digits(num);

    if digit % 2 != 0 {
        return false;
    }

    let half = (digit / 2) as u32;
    let ten_pow_half = 10u64.pow(half);

    num / ten_pow_half == num % ten_pow_half
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_is_invalid_id() {
        assert!(is_invalid_id(11));
        assert!(is_invalid_id(22));
        assert!(is_invalid_id(998998));
        assert!(is_invalid_id(446446));
        assert!(is_invalid_id(1188511885));
        assert!(!is_invalid_id(95));
        assert!(!is_invalid_id(101));
        assert!(!is_invalid_id(1011));
        assert!(!is_invalid_id(1188511890));
    }

    #[test]
    fn test_part1() {
        let ranges = parse(INPUT);
        assert_eq!(part1(&ranges), 1227775554);
    }
}
