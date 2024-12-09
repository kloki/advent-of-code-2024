use toolkit::input::get_input;

pub fn unpack(input: String) -> String {}

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
    fn test_9_step1() {
        assert_eq!(
            "00...111...2...333.44.5555.6666.777.88889i9",
            &unpack("2333133121414131402".to_string())
        )
    }
}
