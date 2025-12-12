fn main() {
    let input = include_str!("../../../inputs/day05.txt");
    let database = parse(input);

    println!("Part 1: {}", part1(&database));
}

fn part1(database: &Database) -> u32 {
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

fn parse(input: &str) -> Database {
    let mut sections = input.split("\n\n");

    let ranges_section = sections.next().unwrap();
    let ids_section = sections.next().unwrap();

    let fresh_ranges = ranges_section
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let start = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();
            Range { start, end }
        })
        .collect::<Vec<Range>>();

    let available_ids = ids_section
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u64>>();

    Database {
        fresh_ranges,
        available_ids,
    }
}

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
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
}
