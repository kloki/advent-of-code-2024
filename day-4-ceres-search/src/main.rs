use toolkit::{
    grid::{Direction, Grid},
    input::get_input,
};

fn build_grid(input: String) -> Grid<char> {
    Grid::new(
        input
            .trim()
            .split("\n")
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>(),
    )
    .unwrap()
}

fn contains_xmas(input: Vec<char>) -> bool {
    input.len() > 3 && input[..4] == ['X', 'M', 'A', 'S']
}

fn count_xmas(grid: Grid<char>) -> usize {
    let mut result = 0;
    for (coor, char) in grid.into_iter() {
        if *char == 'X' {
            for d in Direction::all() {
                if contains_xmas(
                    grid.march(&coor, d)
                        .iter()
                        .map(|(_, value)| **value)
                        .collect(),
                ) {
                    result += 1;
                }
            }
        }
    }
    result
}

// 012
// 3 4
// 567
fn contains_x_mas(input: Vec<char>) -> bool {
    if input.len() < 8 {
        return false;
    }
    let is_diagonal_down = input[0] == 'M' && input[7] == 'S' || input[0] == 'S' && input[7] == 'M';
    let is_diagonal_up = input[5] == 'M' && input[2] == 'S' || input[5] == 'S' && input[2] == 'M';
    is_diagonal_down && is_diagonal_up
}

fn count_x_mas(grid: Grid<char>) -> usize {
    let mut result = 0;
    for (coor, char) in grid.into_iter() {
        if *char == 'A'
            && contains_x_mas(
                grid.get_surrounding(&coor)
                    .iter()
                    .map(|(_, value)| **value)
                    .collect(),
            )
        {
            result += 1;
        }
    }
    result
}

fn main() {
    let contents = get_input();
    let grid = build_grid(contents);
    println!("Solution 1 {}", count_xmas(grid.clone()));
    println!("Solution 2 {}", count_x_mas(grid));
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    #[test]
    fn test_4_count_xmas() {
        let grid = build_grid(TEST_INPUT.to_string());
        assert_eq!(count_xmas(grid), 18)
    }
    #[test]
    fn test_4_count_x_mas() {
        let grid = build_grid(TEST_INPUT.to_string());
        assert_eq!(count_x_mas(grid), 9)
    }
}
