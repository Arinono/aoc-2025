use std::str::FromStr;

advent_of_code::solution!(3);

type Battery = u8;
#[derive(Debug, Clone)]
struct Bank(Vec<Battery>);

impl From<&str> for Bank {
    fn from(input: &str) -> Self {
        let mut bank = vec![];
        for c in input.chars() {
            bank.push(c.to_digit(10).unwrap() as Battery);
        }
        Self(bank)
    }
}

impl FromStr for Bank {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(input))
    }
}

impl Bank {
    fn batteries(&self) -> Vec<Battery> {
        self.0.clone()
    }

    // fn jolts(&self) -> u64 {
    //     let batteries = self.batteries();
    //
    //     let len = batteries.len();
    //     let a = batteries.iter().take(len - 1).max().unwrap();
    //     let index = batteries.iter().position(|b| *b == *a).unwrap();
    //     let b = batteries.iter().skip(index + 1).max().unwrap();
    //
    //     format!("{}{}", a, b).parse::<u64>().unwrap()
    // }

    fn jolts(&self, to_retain: usize) -> u64 {
        let len = self.batteries().len();
        let mut stack: Vec<Battery> = vec![];
        let mut to_remove = len - to_retain;

        self.batteries().iter().for_each(|num| {
            while stack.len() > 0 && to_remove > 0 && stack.last() < Some(num) {
                stack.pop();
                to_remove -= 1;
            }
            stack.push(*num);
        });

        stack
            .iter()
            .take(to_retain)
            .map(|num| num.to_string())
            .collect::<Vec<_>>()
            .join("")
            .parse::<u64>()
            .unwrap()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let banks = input.lines().map(Bank::from).collect::<Vec<_>>();
    let jolts = banks.iter().map(|b| b.jolts(2)).collect::<Vec<_>>();
    let sum = jolts.iter().sum::<u64>();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let banks = input.lines().map(Bank::from).collect::<Vec<_>>();
    let jolts = banks.iter().map(|b| b.jolts(12)).collect::<Vec<_>>();
    let sum = jolts.iter().sum::<u64>();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bank() {
        let input = "987654321111111";
        let bank = Bank::from(input);
        assert_eq!(
            bank.batteries(),
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]
        );
    }

    #[test]
    fn test_parse_banks() {
        let input: &str = &advent_of_code::template::read_file("examples", DAY);
        let banks = input.lines().map(Bank::from).collect::<Vec<_>>();
        dbg!(&banks);
        assert_eq!(banks.len(), 4);
        assert_eq!(
            banks[0].batteries(),
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]
        );
        assert_eq!(
            banks[1].batteries(),
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]
        );
        assert_eq!(
            banks[2].batteries(),
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]
        );
        assert_eq!(
            banks[3].batteries(),
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]
        );
    }

    #[test]
    fn test_bank_jolts() {
        let input = "987654321111111";
        let bank = Bank::from(input);
        let jotls = bank.jolts(2);
        assert_eq!(jotls, 98);

        let input = "811111111111119";
        let bank = Bank::from(input);
        let jotls = bank.jolts(2);
        assert_eq!(jotls, 89);

        let input = "987654321111111";
        let bank = Bank::from(input);
        let jotls = bank.jolts(12);
        assert_eq!(jotls, 987654321111);

        let input = "811111111111119";
        let bank = Bank::from(input);
        let jotls = bank.jolts(12);
        assert_eq!(jotls, 811111111119);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
