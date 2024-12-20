use toolkit::input::get_input;

fn main() {
    let contents = get_input();
    println!("{}", contents);
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
hello world
";
    #[test]
    fn test_0_cookie_cutter() {
        assert_ne!(TEST_INPUT, get_input())
    }
}
