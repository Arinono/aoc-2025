advent_of_code::solution!(6);
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug, PartialEq, Clone)]
struct Problem {
    numbers: Vec<u64>,
    op: Op,
}

#[derive(Debug, PartialEq, Clone)]
struct Problems(Vec<Problem>);
#[derive(Debug, PartialEq, Clone)]
struct ProblemsRtl(Vec<Problem>);

impl From<&str> for Problems {
    fn from(input: &str) -> Self {
        let mut problems = Vec::new();

        for (idx, line) in input.lines().enumerate() {
            if line.is_empty() {
                continue;
            }

            if idx == 0 {
                for num in line.split_whitespace().map(|s| s.parse().unwrap()) {
                    problems.push(Problem {
                        numbers: vec![num],
                        op: Op::Add,
                    });
                }
            } else {
                for (col, val) in line.trim().split_whitespace().enumerate() {
                    if let Ok(num) = val.parse() {
                        problems[col].numbers.push(num);
                    } else {
                        problems[col].op = match val {
                            "+" => Op::Add,
                            "*" => Op::Mul,
                            _ => unreachable!(),
                        };
                    }
                }
            }
        }

        Self(problems)
    }
}

impl FromStr for Problems {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl Problems {
    fn do_homework(&self) -> Vec<u64> {
        self.0
            .iter()
            .map(|p| match p.op {
                Op::Add => p.numbers.iter().sum(),
                Op::Mul => p.numbers.iter().product(),
            })
            .collect()
    }
}

impl From<&str> for ProblemsRtl {
    fn from(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let op_line = lines.last().expect("No operation line");
        let number_lines = &lines[..lines.len() - 1];
        let width = number_lines.iter().map(|l| l.len()).max().unwrap_or(0);
        let mut problems = Vec::new();
        let mut problem_numbers: Vec<u64> = Vec::new();

        for col in (0..width).rev() {
            let mut numbers: Vec<char> = Vec::new();

            for row in number_lines {
                if let Some(ch) = row.chars().nth(col) {
                    numbers.push(ch);
                }
            }
            let num_str = numbers.iter().collect::<String>();
            if num_str.trim().is_empty() {
                problems.push(Problem {
                    numbers: problem_numbers.clone(),
                    op: Op::Add,
                });
                problem_numbers.clear();
            } else {
                let num: u64 = num_str.trim().parse().unwrap();
                problem_numbers.push(num);
            }
        }
        problems.push(Problem {
            numbers: problem_numbers,
            op: Op::Add,
        });

        for (col, op) in op_line.split_whitespace().rev().enumerate() {
            match op {
                "+" => {
                    problems[col].op = Op::Add;
                }
                "*" => {
                    problems[col].op = Op::Mul;
                }
                _ => unreachable!(),
            }
        }

        Self(problems)
    }
}

impl FromStr for ProblemsRtl {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl ProblemsRtl {
    fn do_homework(&self) -> Vec<u64> {
        self.0
            .iter()
            .map(|p| match p.op {
                Op::Add => p.numbers.iter().sum(),
                Op::Mul => p.numbers.iter().product(),
            })
            .collect()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let problems = Problems::from(input);
    let results = problems.do_homework();
    Some(results.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let problems = ProblemsRtl::from(input);
    let results = problems.do_homework();
    Some(results.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> String {
        advent_of_code::template::read_file("examples", DAY)
    }

    #[test]
    fn test_parse_problems() {
        let problems = Problems::from(input().as_ref());
        assert_eq!(problems.0.len(), 4);
        assert_eq!(problems.0[0].numbers.len(), 3);
        assert_eq!(problems.0[0].numbers, vec![123, 45, 6]);
        assert_eq!(problems.0[0].op, Op::Mul);
        assert_eq!(problems.0[1].numbers, vec![328, 64, 98]);
        assert_eq!(problems.0[1].op, Op::Add);
    }

    #[test]
    fn test_do_homework() {
        let problems = Problems::from(input().as_ref());
        let results = problems.do_homework();
        assert_eq!(results.len(), 4);
        assert_eq!(results[0], 33210);
        assert_eq!(results[1], 490);
        assert_eq!(results[2], 4243455);
        assert_eq!(results[3], 401);
    }

    #[test]
    fn test_parse_rtl_problems() {
        let problems = ProblemsRtl::from(input().as_ref());
        assert_eq!(problems.0.len(), 4);
        assert_eq!(problems.0[0].numbers.len(), 3);
        assert_eq!(problems.0[0].numbers, vec![4, 431, 623]);
        assert_eq!(problems.0[0].op, Op::Add);
        assert_eq!(problems.0[1].numbers, vec![175, 581, 32]);
        assert_eq!(problems.0[1].op, Op::Mul);
    }

    #[test]
    fn test_do_homework_rtl() {
        let problems = ProblemsRtl::from(input().as_ref());
        let results = problems.do_homework();
        assert_eq!(results.len(), 4);
        assert_eq!(results[0], 1058);
        assert_eq!(results[1], 3253600);
        assert_eq!(results[2], 625);
        assert_eq!(results[3], 8544);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&input());
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&input());
        assert_eq!(result, Some(3263827));
    }
}
