use std::{collections::HashMap, iter::zip};

use toolkit::input::get_input;

fn parse_content(input: String) -> (Vec<isize>, Vec<isize>) {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let (left, right) = line.trim().split_once("   ").unwrap();
            (
                left.parse::<isize>().unwrap(),
                right.parse::<isize>().unwrap(),
            )
        })
        .unzip()
}

fn find_total_distance(mut left: Vec<isize>, mut right: Vec<isize>) -> isize {
    left.sort();
    right.sort();
    zip(left, right).map(|(l, r)| (l - r).abs()).sum()
}

fn find_similarity_score(left: Vec<isize>, right: Vec<isize>) -> isize {
    let mut counts_right: HashMap<isize, isize> = HashMap::new();
    for r in right {
        let counter = counts_right.entry(r).or_insert(0);
        *counter += 1
    }
    left.iter()
        .map(|l| l * counts_right.get(l).unwrap_or(&0))
        .sum()
}

fn main() {
    let contents = get_input();
    let (left, right) = parse_content(contents);

    println!(
        "Solution 1: {}",
        find_total_distance(left.clone(), right.clone())
    );
    println!("Solution 2: {}", find_similarity_score(left, right));
}
#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_historian_hysteria_total_distance() {
        let (left, right) = parse_content(TEST_INPUT.to_string());
        assert_eq!(find_total_distance(left, right), 11)
    }

    #[test]
    fn test_historian_hysteria_similarity_score() {
        let (left, right) = parse_content(TEST_INPUT.to_string());
        assert_eq!(find_similarity_score(left, right), 31)
    }
}
