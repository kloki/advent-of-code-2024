use toolkit::input::get_input;

fn unpack(input: String) -> String {
    let numbers: Vec<usize> = input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();

    let mut result = "".to_string();
    for (id, i) in (0..(numbers.len())).step_by(2).enumerate() {
        result.push_str(&id.to_string().repeat(numbers[i]));
        if i != numbers.len() - 1 {
            result.push_str(&'.'.to_string().repeat(numbers[i + 1]));
        }
    }

    result.to_string()
}

fn compact(input: String) -> String {
    let mut result = "".to_string();
    let chars: Vec<char> = input.chars().collect();
    let mut back_index = chars.len();
    let total = chars.iter().filter(|x| **x != '.').count();

    for (index, c) in chars.iter().enumerate() {
        if index >= total {
            break;
        }

        if *c == '.' {
            back_index -= 1;
            while chars[back_index] == '.' {
                back_index -= 1;
            }
            result.push(chars[back_index]);
        } else {
            result.push(*c);
        }
    }

    result
}

fn checksum(input: String) -> usize {
    let numbers: Vec<usize> = input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();

    numbers.iter().enumerate().map(|(i, x)| i * x).sum()
}

fn main() {
    let contents = get_input();

    println!("Solution 1 {}", checksum(compact(unpack(contents))));
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9_step1() {
        assert_eq!("0..111....22222", &unpack("12345".to_string()));
        assert_eq!(
            "00...111...2...333.44.5555.6666.777.888899",
            &unpack("2333133121414131402".to_string())
        )
    }

    #[test]
    fn test_9_step2() {
        assert_eq!("022111222", &compact("0..111....22222".to_string()));
        assert_eq!(
            "0099811188827773336446555566",
            &compact("00...111...2...333.44.5555.6666.777.888899".to_string())
        )
    }
    #[test]
    fn test_9_step3() {
        assert_eq!(1928, checksum("0099811188827773336446555566".to_string()))
    }
}
