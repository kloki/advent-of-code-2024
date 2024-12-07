use toolkit::{
    grid::{printer::GridPrinter, Direction, Grid},
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
                    grid.cast_ray(&coor, d)
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
fn contains_x_mas(input: &[char]) -> bool {
    if input.len() < 8 {
        return false;
    }
    let is_horizontal = input[3] == 'M' && input[4] == 'S' || input[3] == 'S' && input[4] == 'M';
    let is_vertical = input[1] == 'M' && input[6] == 'S' || input[1] == 'S' && input[6] == 'M';
    is_horizontal && is_vertical
}

fn contains_x_mas_slant(input: &[char]) -> bool {
    if input.len() < 8 {
        return false;
    }
    let is_diagonal_down = input[0] == 'M' && input[7] == 'S' || input[0] == 'S' && input[7] == 'M';
    let is_diagonal_up = input[5] == 'M' && input[2] == 'S' || input[5] == 'S' && input[2] == 'M';
    is_diagonal_down && is_diagonal_up
}

fn printer(input: &[char], diagonal: bool) {
    let grid: Grid<char> = Grid::new(vec![
        vec![input[0], input[1], input[2]],
        vec![input[3], 'A', input[4]],
        vec![input[5], input[6], input[7]],
    ])
    .unwrap();
    if diagonal {
        println!(
            "{}\n",
            GridPrinter::new(grid)
                .mark_red((0, 0).into())
                .mark_red((2, 2).into())
                .mark_red((0, 2).into())
                .mark_red((2, 0).into())
                .print()
        )
    } else {
        println!(
            "{}\n",
            GridPrinter::new(grid)
                .mark_red((0, 1).into())
                .mark_red((2, 1).into())
                .mark_red((1, 2).into())
                .mark_red((1, 0).into())
                .print()
        )
    }
}

fn count_x_mas(grid: Grid<char>) -> usize {
    let mut result = 0;
    for (coor, char) in grid.into_iter() {
        if *char == 'A' {
            let surrounding: Vec<char> = grid
                .get_surrounding(&coor)
                .iter()
                .map(|(_, value)| **value)
                .collect();
            if contains_x_mas(&surrounding) {
                result += 1;
                printer(&surrounding, false);
            }
            if contains_x_mas_slant(&surrounding) {
                result += 1;
                printer(&surrounding, true);
            }
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
