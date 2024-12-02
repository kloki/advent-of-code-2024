use toolkit::input::get_input;

fn main() {
    let contents = get_input();
    println!("{}", contents);
}

pub fn is_safe(input: Vec<isize>) -> bool {
    let diffs: Vec<isize> = input.windows(2).map(|d| d[0] - d[1]).collect();
    diffs
        .iter()
        .filter(|x| *x < &3 && *x > &0)
        .collect::<Vec<_>>()
        .is_empty()
        || diffs
            .iter()
            .filter(|x| *x > &-3 && *x < &0)
            .collect::<Vec<_>>()
            .is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_2_red_nosed_reports() {
        assert!(is_safe(vec![7, 6, 4, 2, 1]));
    }
}
