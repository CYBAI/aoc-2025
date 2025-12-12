fn main() {
    let input = include_str!("../../../inputs/day05.txt");
    let database = parse(input);

    println!("Part 1: {}", part1(&database));
    println!("Part 2: {}", part2(&database));
}

fn part1(database: &Database) -> u64 {
    let mut count = 0;

    for id in database.available_ids.iter() {
        for range in database.fresh_ranges.iter() {
            if id >= &range.start && id <= &range.end {
                count += 1;
                break;
            }
        }
    }

    count
}

fn part2(database: &Database) -> u64 {
    database
        .fresh_ranges
        .iter()
        .map(|r| r.end - r.start + 1)
        .sum::<u64>()
}

fn parse(input: &str) -> Database {
    let mut sections = input.split("\n\n");

    let ranges_section = sections.next().unwrap();
    let ids_section = sections.next().unwrap();

    let mut raw_ranges = ranges_section
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let start = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();
            Range { start, end }
        })
        .collect::<Vec<Range>>();

    raw_ranges.sort();

    let merged_fresh_ranges = raw_ranges
        .into_iter()
        .fold(Vec::<Range>::new(), |mut acc, r| {
            if let Some(last) = acc.last_mut() {
                if r.start <= last.end {
                    last.end = last.end.max(r.end);
                    return acc;
                }
            }
            acc.push(r);
            acc
        });

    let available_ids = ids_section
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u64>>();

    Database {
        fresh_ranges: merged_fresh_ranges,
        available_ids,
    }
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Range {
    start: u64,
    end: u64,
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let start = self.start.cmp(&other.start);

        if start == std::cmp::Ordering::Equal {
            Some(self.end.cmp(&other.end))
        } else {
            Some(start)
        }
    }
}

struct Database {
    fresh_ranges: Vec<Range>,
    available_ids: Vec<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        let database = parse(INPUT);
        assert_eq!(part1(&database), 3);
    }

    #[test]
    fn test_part2() {
        let mut database = parse(INPUT);
        assert_eq!(part2(&mut database), 14);
    }
}
