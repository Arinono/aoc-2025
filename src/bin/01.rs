use std::str::FromStr;

advent_of_code::solution!(1);

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

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum_zeros = 0u16;
    let mut rotation = 50i16;
    let directions = parse_directions(input);

    for direction in directions {
        rotation = match direction {
            Direction::Left(value) => rotation - (value as i16),
            Direction::Right(value) => rotation + (value as i16),
        };
        if rotation % 100 == 0 {
            sum_zeros += 1;
        }
    }
    Some(sum_zeros as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ticks = 0u64;
    let mut rotation = 50i16;
    let directions = parse_directions(input);

    for direction in directions {
        let start = rotation;
        rotation = match direction {
            Direction::Left(value) => rotation - (value as i16),
            Direction::Right(value) => rotation + (value as i16),
        };
        if start < rotation {
            let first = ((start + 1) as f32 / 100.0).ceil() as i16 * 100;
            if first <= rotation {
                ticks += ((rotation - first) / 100 + 1) as u64;
            }
        } else {
            let last = ((start - 1) as f32 / 100.0).floor() as i16 * 100;
            if last >= rotation {
                ticks += ((last - rotation) / 100 + 1) as u64;
            }
        }
    }

    Some(ticks)
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
        assert_eq!(result, Some(6));
    }
}
