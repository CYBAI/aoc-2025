use std::iter::zip;

fn main() {
    let input = include_str!("../../../inputs/day06.txt");

    println!("Part 1: {}", part1(parse(input)));
}

fn part1(math_problems: MathProblems) -> u64 {
    let MathProblems { columns } = math_problems;

    columns
        .iter()
        .map(|col| {
            col.raw_nums
                .iter()
                .map(|xs| xs.trim().parse().unwrap())
                .fold(col.op.mempty(), |acc, n| col.op.calc(acc, n))
        })
        .sum()
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug)]
struct Column {
    width: usize,
    op: Operator,
    raw_nums: Vec<String>,
}

struct MathProblems {
    columns: Vec<Column>,
}

fn parse(input: &str) -> MathProblems {
    let input = input.trim_start();

    let mut columns = initialize_columns(input.lines().last().unwrap());
    let widths = columns.iter().map(|col| col.width).collect::<Vec<usize>>();

    for line in input.lines().take(input.lines().count() - 1) {
        let mut start = 0;
        for (col, &width) in zip(&mut columns, &widths) {
            let end = start + width;
            col.raw_nums.push(line[start..end].to_string());
            start = end + 1;
        }
    }

    MathProblems { columns }
}

fn initialize_columns(ops_line: &str) -> Vec<Column> {
    let mut columns = vec![];

    let mut chars = ops_line.chars().peekable();
    while let Some(c) = chars.next() {
        if let Some(op) = Operator::from_char(c) {
            let mut current_width = 1;
            let mut found_next_op = false;

            while let Some(&next_char) = chars.peek() {
                if next_char.is_whitespace() {
                    current_width += 1;
                    chars.next();
                } else {
                    found_next_op = true;
                    break;
                }
            }

            columns.push(Column {
                width: if found_next_op {
                    current_width - 1
                } else {
                    current_width
                },
                op,
                raw_nums: vec![],
            });
        }
    }

    columns
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
        assert_eq!(part1(parse(INPUT)), 4277556);
    }
}
