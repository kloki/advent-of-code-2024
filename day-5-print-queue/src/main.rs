use std::collections::HashMap;

use toolkit::input::get_input;

#[derive(Debug)]
struct Rule {
    pub before: usize,
    pub after: usize,
}

#[derive(Debug, Clone)]
struct Update {
    pub middle: usize,
    rank: HashMap<usize, usize>,
}

impl Update {
    pub fn new(input: Vec<usize>) -> Self {
        let middle = input[input.len() / 2];
        let mut rank = HashMap::new();
        for (i, value) in input.iter().enumerate() {
            rank.insert(*value, i);
        }

        Self { middle, rank }
    }

    pub fn is_correct(&self, rules: &Vec<Rule>) -> bool {
        for rule in rules {
            match (self.rank.get(&rule.before), self.rank.get(&rule.after)) {
                (Some(before), Some(after)) if after < before => return false,
                _ => {}
            }
        }
        true
    }

    pub fn fix(&self, rules: &Vec<Rule>) -> Self {
        self.clone()
    }
}

fn main() {
    let contents = get_input();

    let (rules, updates) = parse_input(contents.to_string());
    let score: usize = updates
        .iter()
        .filter(|u| u.is_correct(&rules))
        .map(|u| u.middle)
        .sum();
    println!("Solution 1 {}", score);

    let score: usize = updates
        .iter()
        .filter(|u| !u.is_correct(&rules))
        .map(|u| u.fix(&rules))
        .map(|u| u.middle)
        .sum();
    println!("Solution 2 {}", score);
}

fn parse_input(input: String) -> (Vec<Rule>, Vec<Update>) {
    let split = input.trim().split("\n\n").collect::<Vec<_>>();
    let rules = split[0]
        .split("\n")
        .map(|line| {
            let splitted = line.split("|").collect::<Vec<_>>();
            Rule {
                before: splitted[0].parse().unwrap(),
                after: splitted[1].parse().unwrap(),
            }
        })
        .collect::<Vec<Rule>>();
    let updates = split[1]
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(Update::new)
        .collect::<Vec<_>>();
    (rules, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
    #[test]
    fn test_5_middle_number_correct() {
        let (rules, updates) = parse_input(TEST_INPUT.to_string());

        let score: usize = updates
            .iter()
            .filter(|u| u.is_correct(&rules))
            .map(|u| u.middle)
            .sum();
        assert_eq!(score, 143)
    }

    #[test]
    fn test_5_middle_number_wrong() {
        let (rules, updates) = parse_input(TEST_INPUT.to_string());

        let score: usize = updates
            .iter()
            .filter(|u| !u.is_correct(&rules))
            .map(|u| u.fix(&rules))
            .map(|u| u.middle)
            .sum();
        assert_eq!(score, 123)
    }
}
