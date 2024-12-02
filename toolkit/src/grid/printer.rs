use std::collections::HashMap;

use colorize::AnsiColor;

use super::models::{Coordinate, Grid};

#[derive(Clone, Copy, Debug)]
enum Color {
    Red,
    Yellow,
    Green,
    None,
    White,
}

impl Color {
    pub fn apply(&self, input: String) -> String {
        match self {
            Color::Red => input.redb(),
            Color::Yellow => input.black().yellowb(),
            Color::Green => input.black().greenb(),
            Color::White => input.black().greyb(),
            _ => input,
        }
    }
}

pub struct GridPrinter<T>
where
    T: std::fmt::Display,
{
    grid: Grid<T>,
    colored: HashMap<Coordinate, Color>,
    checkered: bool,
}

impl<T> GridPrinter<T>
where
    T: std::fmt::Display,
{
    pub fn new(grid: Grid<T>) -> Self {
        Self {
            grid,
            colored: HashMap::new(),
            checkered: false,
        }
    }
    pub fn checkered(mut self) -> Self {
        self.checkered = true;
        self
    }

    pub fn mark_red(mut self, coor: Coordinate) -> Self {
        self.colored.insert(coor, Color::Red);
        self
    }

    pub fn mark_yellow(mut self, coor: Coordinate) -> Self {
        self.colored.insert(coor, Color::Yellow);
        self
    }

    pub fn mark_green(mut self, coor: Coordinate) -> Self {
        self.colored.insert(coor, Color::Green);
        self
    }

    fn get_color_background(&self, coor: Coordinate) -> Color {
        if self.checkered && coor.row % 2 == coor.column % 2 {
            return Color::White;
        }

        Color::None
    }
    fn get_color(&self, coor: Coordinate) -> Color {
        *self
            .colored
            .get(&coor)
            .unwrap_or(&self.get_color_background(coor))
    }

    pub fn print(&self) -> String {
        let mut current_row = 0;
        let mut output = "".to_string();
        for (coor, grid_item) in self.grid.into_iter() {
            if coor.row != current_row {
                output.push('\n');
                current_row = coor.row
            }
            output.push_str(&self.get_color(coor).apply(grid_item.to_string()))
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_colors() {
        let grid: Grid<usize> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
            .try_into()
            .unwrap();

        let result = GridPrinter::new(grid)
            .mark_red((0, 0).into())
            .mark_yellow((1, 1).into())
            .mark_green((2, 2).into())
            .print();

        assert_eq!("\u{1b}[41m1\u{1b}[0;39;49m23\n4\u{1b}[43;30m5\u{1b}[0;39;49m6\n78\u{1b}[42;30m9\u{1b}[0;39;49m", result)
    }

    #[test]
    fn test_checkers() {
        let grid: Grid<usize> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
            .try_into()
            .unwrap();

        let result = GridPrinter::new(grid).checkered().print();
        assert_eq!("\u{1b}[47;30m1\u{1b}[0;39;49m2\u{1b}[47;30m3\u{1b}[0;39;49m\n4\u{1b}[47;30m5\u{1b}[0;39;49m6\n\u{1b}[47;30m7\u{1b}[0;39;49m8\u{1b}[47;30m9\u{1b}[0;39;49m", result)
    }
}
