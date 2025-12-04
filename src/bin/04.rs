use std::{fmt::Display, str::FromStr};

advent_of_code::solution!(4);

#[derive(Debug, Clone, PartialEq)]
enum Spot {
    Roll,
    Empty,
    AccessibleRoll,
}

impl Display for Spot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spot::Roll => write!(f, "@"),
            Spot::Empty => write!(f, "."),
            Spot::AccessibleRoll => write!(f, "x"),
        }
    }
}

#[derive(Clone)]
struct Grid(Vec<Vec<Spot>>);

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let mut grid = vec![];
        for line in input.lines() {
            if line.is_empty() {
                continue;
            }
            let line = line.chars().collect::<Vec<_>>();
            grid.push(
                line.into_iter()
                    .map(|c| match c {
                        '.' => Spot::Empty,
                        '@' => Spot::Roll,
                        _ => unreachable!(),
                    })
                    .collect(),
            );
        }

        Grid(grid)
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(input))
    }
}

const DIRECTIONS: [(i32, i32); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

impl Grid {
    fn rows_mut(&mut self) -> &mut Vec<Vec<Spot>> {
        &mut self.0
    }

    fn adjacent(&self, x: usize, y: usize) -> Vec<Spot> {
        let mut adjacents: Vec<Spot> = vec![];

        for (dx, dy) in DIRECTIONS {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            if new_x >= 0
                && new_x < self.0.len() as i32
                && new_y >= 0
                && new_y < self.0[0].len() as i32
            {
                let new_row = self.0.get(new_y as usize).unwrap();
                let new_spot = new_row.get(new_x as usize).unwrap();
                adjacents.push(new_spot.clone());
            }
        }

        adjacents
    }

    fn accessible_rolls(&mut self) {
        let cloned_self = self.clone();

        for (y, row) in self.0.iter_mut().enumerate() {
            for (x, spot) in row.iter_mut().enumerate() {
                match spot {
                    Spot::Roll => {
                        let adjacent = cloned_self.adjacent(x, y);
                        if adjacent.iter().filter(|s| *s == &Spot::Roll).count() < 4 {
                            *spot = Spot::AccessibleRoll;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn remove_accessible_rolls(&mut self) -> usize {
        let mut removed = 0;

        for row in self.rows_mut().iter_mut() {
            for spot in row.iter_mut() {
                match spot {
                    Spot::AccessibleRoll => {
                        *spot = Spot::Empty;
                        removed += 1;
                    }
                    _ => {}
                }
            }
        }

        removed
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = Grid::from(input);
    grid.accessible_rolls();
    let sum = grid
        .rows_mut()
        .iter()
        .flat_map(|row| row.iter().filter(|s| *s == &Spot::AccessibleRoll))
        .count();

    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::from(input);
    grid.accessible_rolls();
    let mut removed = grid.remove_accessible_rolls();
    let mut total = 0;

    total += removed;

    while removed > 0 {
        grid.accessible_rolls();
        removed = grid.remove_accessible_rolls();
        total += removed;
    }

    Some(total as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grid() {
        let input: &str = &advent_of_code::template::read_file("examples", DAY);
        let mut grid = Grid::from(input);
        assert_eq!(grid.rows_mut().len(), 10);
        assert_eq!(grid.rows_mut()[0].len(), 10);
        assert_eq!(
            grid.rows_mut()[0],
            vec![
                Spot::Empty,
                Spot::Empty,
                Spot::Roll,
                Spot::Roll,
                Spot::Empty,
                Spot::Roll,
                Spot::Roll,
                Spot::Roll,
                Spot::Roll,
                Spot::Empty,
            ]
        );
    }

    #[test]
    fn test_adjacent() {
        let input: &str = &advent_of_code::template::read_file("examples", DAY);
        let grid = Grid::from(input);
        let adjacent = grid.adjacent(0, 0);
        assert_eq!(adjacent.len(), 3);
        assert_eq!(adjacent[0], Spot::Empty);
        assert_eq!(adjacent[1], Spot::Roll);
        assert_eq!(adjacent[2], Spot::Roll);

        let adjacent = grid.adjacent(7, 0);
        assert_eq!(adjacent.len(), 5);
        assert_eq!(adjacent[0], Spot::Roll);
        assert_eq!(adjacent[1], Spot::Roll);
        assert_eq!(adjacent[2], Spot::Empty);
        assert_eq!(adjacent[3], Spot::Roll);
        assert_eq!(adjacent[4], Spot::Roll);
    }

    #[test]
    fn test_find_accessible_rolls() {
        let input: &str = &advent_of_code::template::read_file("examples", DAY);
        let mut grid = Grid::from(input);
        grid.accessible_rolls();
        assert_eq!(
            grid.rows_mut()[0]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(""),
            "..xx.xx@x."
        );
        assert_eq!(
            grid.rows_mut()[1]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(""),
            "x@@.@.@.@@"
        );
    }

    #[test]
    fn test_remove_accessible_rolls() {
        let input: &str = &advent_of_code::template::read_file("examples", DAY);
        let mut grid = Grid::from(input);
        grid.accessible_rolls();
        let removed = grid.remove_accessible_rolls();

        assert_eq!(
            grid.rows_mut()[0]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(""),
            ".......@.."
        );
        assert_eq!(
            grid.rows_mut()[1]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(""),
            ".@@.@.@.@@"
        );
        assert_eq!(removed, 13);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
