use std::str::FromStr;

advent_of_code::solution!(1);

const MAX_ROTATIONS: u8 = 99;
const MIN_ROTATIONS: u8 = 0;
const START: u8 = 50;

/// L50 means left 50 steps
/// R50 means right 50 steps
#[derive(Debug, PartialEq)]
enum Direction {
    Left(u16),
    Right(u16),
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s[0..1].to_ascii_lowercase().as_str() {
            "l" => Direction::Left(s[1..].parse().unwrap()),
            "r" => Direction::Right(s[1..].parse().unwrap()),
            _ => panic!("Invalid direction"),
        }
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Direction::from(s))
    }
}

fn parse_directions(input: &str) -> Vec<Direction> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect()
}

fn compute_rotations(directions: &[Direction]) -> Vec<u8> {
    let mut rotations: Vec<u8> = vec![START];

    for direction in directions {
        match direction {
            Direction::Left(steps) => {
                let mut target: i16 = (*rotations.last().unwrap() as i16) - (*steps as i16);
                while target < (MIN_ROTATIONS).into() {
                    target += MAX_ROTATIONS as i16 + 1;
                }
                rotations.push(target as u8);
            }
            Direction::Right(steps) => {
                let mut target: i16 = (*rotations.last().unwrap() as i16) + (*steps as i16);
                while target > (MAX_ROTATIONS).into() {
                    target -= MAX_ROTATIONS as i16 + 1;
                }
                rotations.push(target as u8);
            }
        }
    }
    rotations
}

pub fn part_one(input: &str) -> Option<u64> {
    let directions = parse_directions(input);
    let rotations = compute_rotations(&directions);

    let rotations = rotations.into_iter();
    let sum_zeros = rotations.filter(|&x| x == 0).count();

    Some(sum_zeros as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_direction() {
        assert_eq!(Direction::from("L50"), Direction::Left(50));
        assert_eq!(Direction::from("R50"), Direction::Right(50));
    }

    #[test]
    fn test_parse_directions() {
        assert_eq!(
            parse_directions("L50\nR50\nL10\nL5\nR55\nR5\nL15\nL4"),
            vec![
                Direction::Left(50),
                Direction::Right(50),
                Direction::Left(10),
                Direction::Left(5),
                Direction::Right(55),
                Direction::Right(5),
                Direction::Left(15),
                Direction::Left(4),
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
