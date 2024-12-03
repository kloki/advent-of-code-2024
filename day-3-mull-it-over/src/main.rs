use regex::Regex;
use toolkit::input::get_input;

fn main() {
    let contents = get_input();
    println!("Solution 1 {}", calculate_complete_mulls(contents));
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

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_1_mull_it_over() {
        assert_eq!(calculate_complete_mulls(TEST_INPUT.to_string()), 161)
    }
}
