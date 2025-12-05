use std::{ops::RangeInclusive, str::FromStr};

advent_of_code::solution!(5);

type Ingredient = u64;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Inventory {
    fresh: Vec<RangeInclusive<u64>>,
    ingredients: Vec<Ingredient>,
}

impl From<&str> for Inventory {
    fn from(input: &str) -> Self {
        let mut inv: Vec<RangeInclusive<u64>> = Vec::new();
        let mut ingredients: Vec<Ingredient> = Vec::new();
        let mut blank = false;

        for line in input.lines() {
            if line.is_empty() {
                blank = true;
                continue;
            }

            if !blank {
                let (start, end) = line.split_once('-').unwrap();
                inv.push(start.parse().unwrap()..=end.parse().unwrap());
            } else {
                ingredients.push(line.parse().unwrap());
            }
        }

        Self {
            fresh: inv,
            ingredients,
        }
    }
}

impl FromStr for Inventory {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl Inventory {
    fn merge_fresh(&mut self) {
        if self.fresh.is_empty() {
            return;
        }

        self.fresh.sort_by_key(|r| *r.start());

        let mut write_idx = 0;

        for read_idx in 1..self.fresh.len() {
            let (start, end) = (*self.fresh[read_idx].start(), *self.fresh[read_idx].end());

            if start <= self.fresh[write_idx].end() + 1 {
                let new_end = self.fresh[write_idx].end().max(&end);
                self.fresh[write_idx] = *self.fresh[write_idx].start()..=*new_end;
            } else {
                write_idx += 1;
                self.fresh[write_idx] = start..=end;
            }
        }

        self.fresh.truncate(write_idx + 1);
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut inventory = Inventory::from(input);
    inventory.merge_fresh();

    let fresh = inventory
        .ingredients
        .iter()
        .filter(|i| inventory.fresh.iter().any(|r| r.contains(i)));

    Some(fresh.count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut inventory = Inventory::from(input);
    inventory.merge_fresh();
    let mut count = 0;
    for i in &inventory.fresh {
        count += i.end() - i.start() + 1;
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> String {
        advent_of_code::template::read_file("examples", DAY)
    }

    #[test]
    fn test_parse_input() {
        let inventory = Inventory::from(input().as_ref());

        assert_eq!(inventory.fresh, vec![3..=5, 10..=14, 16..=20, 12..=18]);
        assert_eq!(inventory.ingredients, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn test_merge_ranges() {
        let mut inventory = Inventory::from(input().as_ref());
        inventory.merge_fresh();

        assert_eq!(inventory.fresh, vec![3..=5, 10..=20]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&input());
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&input());
        assert_eq!(result, Some(14));
    }
}
