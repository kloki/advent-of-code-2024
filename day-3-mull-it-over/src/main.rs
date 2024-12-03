use regex::Regex;
use toolkit::input::get_input;

fn main() {
    let contents = get_input();
    println!("Solution 1 {}", calculate_complete_mulls(contents.clone()));
    println!(
        "Solution 2 {}",
        calculate_complete_mulls_conditional(contents)
    );
}

#[derive(Debug)]
struct Mul {
    a: usize,
    b: usize,
}

impl Mul {
    pub fn calc(&self) -> usize {
        self.a * self.b
    }
}

fn parse_mulls(input: String) -> Vec<Mul> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut muls = Vec::new();
    for (_, [a, b]) in re.captures_iter(&input).map(|c| c.extract()) {
        muls.push(Mul {
            a: a.parse().unwrap(),
            b: b.parse().unwrap(),
        })
    }
    muls
}

pub fn calculate_complete_mulls(input: String) -> usize {
    let mulls = parse_mulls(input);
    mulls.iter().map(|x| x.calc()).sum()
}

pub fn prune_input(input: String) -> Vec<String> {
    let mut work_input = input;
    let mut result = Vec::new();
    loop {
        match work_input.split_once("don't()") {
            Some((included, remainder)) => {
                result.push(included.to_string());
                work_input = remainder.to_string()
            }
            None => {
                result.push(work_input);
                break;
            }
        }
        match work_input.split_once("do()") {
            Some((_, remainder)) => work_input = remainder.to_string(),
            None => {
                break;
            }
        }
    }
    result
}

pub fn calculate_complete_mulls_conditional(input: String) -> usize {
    prune_input(input)
        .iter()
        .map(|line| calculate_complete_mulls(line.to_string()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const TEST_INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_1_mull_it_over() {
        assert_eq!(calculate_complete_mulls(TEST_INPUT_1.to_string()), 161)
    }

    #[test]
    fn test_2_mull_it_over() {
        assert_eq!(
            calculate_complete_mulls_conditional(TEST_INPUT_2.to_string()),
            48
        )
    }
}
