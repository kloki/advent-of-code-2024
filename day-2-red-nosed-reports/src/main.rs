use itertools::Itertools;
use toolkit::input::get_input;

fn parse_input(input: String) -> Vec<Vec<isize>> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            line.split(" ")
                .map(|i| i.parse::<isize>().unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    let contents = get_input();
    let reports = parse_input(contents);

    println!(
        "Solution 1: {}",
        reports.iter().filter(|x| is_safe(x)).count()
    );
    println!(
        "Solution 2: {}",
        reports.iter().filter(|x| is_safe_dampened(x)).count()
    );
}

pub fn is_safe(input: &Vec<isize>) -> bool {
    let diffs: Vec<isize> = input.windows(2).map(|d| d[0] - d[1]).collect();
    diffs.iter().all(|x| *x < 4 && *x > 0) || diffs.iter().all(|x| *x > -4 && *x < 0)
}
pub fn is_safe_dampened(input: &Vec<isize>) -> bool {
    for report in input.iter().combinations(input.len() - 1) {
        if is_safe(&report.iter().map(|x| **x).collect()) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1_red_nosed_reports() {
        assert!(is_safe(&vec![7, 6, 4, 2, 1]));
        assert!(!is_safe(&vec![1, 2, 7, 8, 9]));
        assert!(!is_safe(&vec![9, 7, 6, 2, 1]));
        assert!(!is_safe(&vec![1, 3, 2, 4, 5]));
        assert!(!is_safe(&vec![8, 6, 4, 4, 1]));
        assert!(is_safe(&vec![1, 3, 6, 7, 9]));
    }

    #[test]
    fn test_2_red_nosed_reports() {
        assert!(is_safe_dampened(&vec![7, 6, 4, 2, 1]));
        assert!(!is_safe_dampened(&vec![1, 2, 7, 8, 9]));
        assert!(!is_safe_dampened(&vec![9, 7, 6, 2, 1]));
        assert!(is_safe_dampened(&vec![1, 3, 2, 4, 5]));
        assert!(is_safe_dampened(&vec![8, 6, 4, 4, 1]));
        assert!(is_safe_dampened(&vec![1, 3, 6, 7, 9]));
    }
}
