use toolkit::input::get_input;

struct Calculation {
    pub result: usize,
    input: Vec<usize>,
}

#[derive(Clone, Copy)]
enum OperatorSet {
    Two,
    Three,
}

impl OperatorSet {
    pub fn combinations(&self, a: usize, b: usize) -> Vec<usize> {
        let mut result = vec![a + b, a * b];
        if let OperatorSet::Three = self {
            result.push((a.to_string() + &b.to_string()).parse().unwrap());
        }
        result
    }
}

impl Calculation {
    pub fn new(result: usize, input: Vec<usize>) -> Self {
        Self { result, input }
    }

    fn build(&self, mut inputs: Vec<usize>, current: usize, operator_set: OperatorSet) -> bool {
        if current > self.result {
            return false;
        }
        match inputs.pop() {
            None => current == self.result,
            Some(value) => operator_set
                .combinations(current, value)
                .iter()
                .any(|x| self.build(inputs.clone(), *x, operator_set)),
        }
    }

    pub fn could_be_true(&self, operator_set: OperatorSet) -> bool {
        let mut inputs = self.input.clone();
        inputs.reverse();
        let current = inputs.pop().unwrap_or_default();
        self.build(inputs, current, operator_set)
    }
}

fn parse_input(input: String) -> Vec<Calculation> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let split = line.split(": ").collect::<Vec<_>>();
            let result: usize = split[0].parse().unwrap();
            let input_options: Vec<usize> = split[1]
                .split(" ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            Calculation::new(result, input_options)
        })
        .collect::<Vec<_>>()
}

fn main() {
    let contents = get_input();
    let calcs = parse_input(contents);

    println!(
        "Solution 1 {}",
        calcs
            .iter()
            .filter(|c| c.could_be_true(OperatorSet::Two))
            .map(|x| x.result)
            .sum::<usize>()
    );
    println!(
        "Solution 2 {}",
        calcs
            .iter()
            .filter(|c| c.could_be_true(OperatorSet::Three))
            .map(|x| x.result)
            .sum::<usize>()
    );
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
    #[test]
    fn test_7_debug() {
        assert!(Calculation::new(190, vec![10, 19]).could_be_true(OperatorSet::Two));
        assert!(Calculation::new(3267, vec![81, 40, 27]).could_be_true(OperatorSet::Two));
        assert!(Calculation::new(292, vec![11, 6, 16, 20]).could_be_true(OperatorSet::Two));
    }

    #[test]
    fn test_7_solution_1() {
        let calcs = parse_input(TEST_INPUT.to_string());

        assert_eq!(
            calcs
                .iter()
                .filter(|c| c.could_be_true(OperatorSet::Two))
                .map(|c| c.result)
                .sum::<usize>(),
            3749
        );
    }

    #[test]
    fn test_7_solution_2() {
        let calcs = parse_input(TEST_INPUT.to_string());

        assert_eq!(
            calcs
                .iter()
                .filter(|c| c.could_be_true(OperatorSet::Three))
                .map(|c| c.result)
                .sum::<usize>(),
            11387
        );
    }
}
