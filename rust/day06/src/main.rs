use std::iter::zip;

fn main() {
    let input = include_str!("../../../inputs/day06.txt");
    let (nums, ops) = parse(input);

    println!("Part 1: {}", part1(&nums, &ops));
}

fn part1(nums: &[Vec<u64>], ops: &[Operator]) -> u64 {
    let transposed: Vec<Vec<u64>> = transpose(nums.to_vec());

    zip(transposed.iter(), ops)
        .into_iter()
        .map(|(col, op)| col.iter().fold(op.mempty(), |acc, &n| op.calc(acc, n)))
        .sum()
}

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
}

impl Operator {
    fn from_char(c: char) -> Option<Operator> {
        match c {
            '+' => Some(Operator::Add),
            '*' => Some(Operator::Mul),
            _ => None,
        }
    }

    fn mempty(&self) -> u64 {
        match self {
            Operator::Add => 0,
            Operator::Mul => 1,
        }
    }

    fn calc(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Add => a + b,
            Operator::Mul => a * b,
        }
    }
}

fn parse(input: &str) -> (Vec<Vec<u64>>, Vec<Operator>) {
    let mut nums = vec![];
    let mut ops = vec![];

    for line in input.trim().lines() {
        let line = line.trim();
        if let Some(c) = line.chars().next()
            && c.is_digit(10)
        {
            let row = line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect::<Vec<u64>>();
            nums.push(row);
        } else {
            ops = line
                .chars()
                .filter_map(|c| Operator::from_char(c))
                .collect::<Vec<Operator>>();
        }
    }

    (nums, ops)
}

fn transpose(matrix: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut iter = matrix.into_iter();

    let first = match iter.next() {
        Some(row) => row,
        None => return vec![],
    };

    let mut acc: Vec<Vec<u64>> = first.into_iter().map(|x| vec![x]).collect();

    for row in iter {
        for (col, item) in zip(&mut acc, row) {
            col.push(item);
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        let (nums, ops) = parse(INPUT);
        assert_eq!(part1(&nums, &ops), 4277556);
    }
}
