use rayon::prelude::*;
use std::{ops::Range, str::FromStr};

advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
struct Ranges(Vec<Range<u64>>);

impl From<&str> for Ranges {
    fn from(input: &str) -> Self {
        let mut ranges = Vec::new();
        let line = input.lines().take(1).next().unwrap();

        line.split(',').for_each(|range| {
            let range = range.split('-').collect::<Vec<&str>>();
            let start = range[0].parse::<u64>().unwrap();
            let end = range[1].parse::<u64>().unwrap();
            ranges.push(start..end);
        });

        Ranges(ranges)
    }
}

impl FromStr for Ranges {
    type Err = Box<dyn std::error::Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(input))
    }
}

trait Dupes
where
    Self: Iterator,
{
    fn get_dupes(&self) -> Vec<u64>;
    fn get_repeats(&self) -> Vec<u64>;
}

impl Dupes for Range<u64> {
    fn get_dupes(&self) -> Vec<u64> {
        (self.start..=self.end)
            .into_par_iter()
            .flat_map(|i| {
                let istr = i.to_string();
                if istr.len() % 2 == 0 {
                    let (left, right) = istr.split_at(i.to_string().len() / 2);
                    if left.parse::<u64>().unwrap() == right.parse::<u64>().unwrap() {
                        return Some(i);
                    }
                }
                None
            })
            .collect::<Vec<u64>>()
    }

    fn get_repeats(&self) -> Vec<u64> {
        (self.start..=self.end)
            .into_par_iter()
            .flat_map(|i| {
                let istr = i.to_string();
                let doubled = format!("{istr}{istr}");

                if doubled[1..doubled.len() - 1].contains(&istr) {
                    return Some(i);
                }

                None
            })
            .collect::<Vec<u64>>()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = Ranges::from(input);

    let ids = ranges
        .0
        .iter()
        .flat_map(|range| range.get_dupes())
        .collect::<Vec<u64>>();

    Some(ids.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = Ranges::from(input);

    let ids = ranges
        .0
        .iter()
        .flat_map(|range| range.get_repeats())
        .collect::<Vec<u64>>();

    Some(ids.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let example: &str = &advent_of_code::template::read_file("examples", DAY);
        let result = Ranges::from(example);
        assert_eq!(
            result,
            Ranges(vec![
                Range { start: 11, end: 22 },
                Range {
                    start: 95,
                    end: 115
                },
                Range {
                    start: 998,
                    end: 1012
                },
                Range {
                    start: 1188511880,
                    end: 1188511890
                },
                Range {
                    start: 222220,
                    end: 222224
                },
                Range {
                    start: 1698522,
                    end: 1698528
                },
                Range {
                    start: 446443,
                    end: 446449
                },
                Range {
                    start: 38593856,
                    end: 38593862
                },
                Range {
                    start: 565653,
                    end: 565659
                },
                Range {
                    start: 824824821,
                    end: 824824827
                },
                Range {
                    start: 2121212118,
                    end: 2121212124
                },
            ])
        );
    }

    #[test]
    fn test_get_dupes() {
        assert_eq!(Range { start: 11, end: 22 }.get_dupes(), vec![11, 22]);
        assert_eq!(
            Range {
                start: 95,
                end: 115
            }
            .get_dupes(),
            vec![99]
        );
        assert_eq!(
            Range {
                start: 998,
                end: 1012
            }
            .get_dupes(),
            vec![1010]
        );
    }

    #[test]
    fn test_get_repeats() {
        assert_eq!(Range { start: 11, end: 22 }.get_repeats(), vec![11, 22]);
        assert_eq!(
            Range {
                start: 95,
                end: 115
            }
            .get_repeats(),
            vec![99, 111]
        );
        assert_eq!(
            Range {
                start: 998,
                end: 1012
            }
            .get_repeats(),
            vec![999, 1010]
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
