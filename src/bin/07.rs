advent_of_code::solution!(7);
use std::{collections::HashMap, fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Spot {
    Start,
    Empty,
    Splitter,
    Tachyon,
}

#[derive(Debug, Clone)]
struct TachyonTimeline {
    index: usize,
    weight: usize,
}

#[derive(Debug, Clone)]
struct Row(Vec<Spot>);

#[derive(Debug)]
struct Grid {
    rows: Vec<Row>,
    row: usize,
    split_count: usize,
    timelines: Vec<TachyonTimeline>,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let rows: Vec<Row> = input
            .lines()
            .map(|line| {
                Row(line
                    .chars()
                    .map(|c| match c {
                        '.' => Spot::Empty,
                        '^' => Spot::Splitter,
                        'S' => Spot::Start,
                        'T' => Spot::Tachyon,
                        _ => unreachable!(),
                    })
                    .collect())
            })
            .collect();

        let (start_pos, _) = rows[0]
            .0
            .iter()
            .enumerate()
            .find(|(_, s)| *s == &Spot::Start)
            .unwrap();

        Self {
            rows,
            row: 0,
            split_count: 0,
            timelines: vec![TachyonTimeline {
                index: start_pos,
                weight: 1,
            }],
        }
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(input))
    }
}

impl Display for Spot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spot::Start => write!(f, "S"),
            Spot::Empty => write!(f, "."),
            Spot::Splitter => write!(f, "^"),
            Spot::Tachyon => write!(f, "|"),
        }
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().map(|s| s.to_string()).collect::<String>()
        )
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.rows
                .iter()
                .map(|r| r.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Grid {
    fn run(&mut self) {
        if self.row == self.rows.len() - 1 {
            return;
        }
        let indices = self.rows[self.row]
            .0
            .iter()
            .enumerate()
            .filter_map(|(i, s)| match s {
                Spot::Start | Spot::Tachyon => Some(i),
                _ => None,
            })
            .collect::<Vec<usize>>();

        for i in indices {
            if self.rows[self.row + 1].0[i] == Spot::Splitter {
                self.split_count += 1;
                self.rows[self.row + 1].0[i - 1] = Spot::Tachyon;
                self.rows[self.row + 1].0[i + 1] = Spot::Tachyon;
                let timeline = self.timelines.iter_mut().find(|t| t.index == i).unwrap();
                timeline.index -= 1;
                let weight = timeline.weight;
                self.timelines.push(TachyonTimeline {
                    index: i + 1,
                    weight: weight,
                });
                self.timelines.sort_by_key(|t| t.index);

                let mut weight_map: HashMap<usize, usize> = HashMap::new();
                for timeline in self.timelines.iter() {
                    *weight_map.entry(timeline.index).or_insert(0) += timeline.weight;
                }

                let mut write_pos = 0;
                for (index, weight) in weight_map.into_iter() {
                    self.timelines[write_pos] = TachyonTimeline { index, weight };
                    write_pos += 1;
                }

                self.timelines.truncate(write_pos);
            } else {
                self.rows[self.row + 1].0[i] = Spot::Tachyon;
            }
        }

        self.row += 1;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = Grid::from(input);
    while grid.row < grid.rows.len() - 1 {
        grid.run();
    }
    Some(grid.split_count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::from(input);
    while grid.row < grid.rows.len() - 1 {
        grid.run();
    }
    Some(grid.timelines.iter().map(|t| t.weight).sum::<usize>() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> String {
        advent_of_code::template::read_file("examples", DAY)
    }

    #[test]
    fn test_parse() {
        let grid = Grid::from(input().as_str());
        assert_eq!(grid.rows.len(), 16);
        assert_eq!(grid.rows[0].to_string(), ".......S.......");
    }

    #[test]
    fn test_run() {
        let mut grid = Grid::from(input().as_str());
        grid.run();
        grid.run();
        assert_eq!(grid.rows[0].to_string(), ".......S.......");
        assert_eq!(grid.rows[1].to_string(), ".......|.......");
        assert_eq!(grid.rows[2].to_string(), "......|^|......");
    }

    #[test]
    fn test_full_run() {
        let mut grid = Grid::from(input().as_str());
        while grid.row < grid.rows.len() - 1 {
            grid.run();
        }
        assert_eq!(grid.split_count, 21);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&input());
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&input());
        assert_eq!(result, Some(40));
    }
}
