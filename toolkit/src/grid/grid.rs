use std::fmt;

use super::{coordinate::Coordinate, direction::Direction, planner::GridPlanner};

#[derive(Clone, Debug)]
pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
    planner: GridPlanner,
}

impl<T> Grid<T> {
    pub fn new(input: Vec<Vec<T>>) -> Result<Self, &'static str> {
        let length = input.len();
        let width = input[0].len();
        if input.iter().filter(|x| x.len() != width).count() != 0 {
            Err("Not all rows have the same length.")
        } else {
            Ok(Grid {
                grid: input,
                planner: GridPlanner::new(length, width),
            })
        }
    }
    pub fn get(&self, coor: &Coordinate) -> Option<&T> {
        if coor.column > self.max_column() || coor.row > self.max_row() {
            return None;
        }
        Some(&self.grid[coor.row][coor.column])
    }
    pub fn get_mut(&mut self, coor: &Coordinate) -> Option<&mut T> {
        if coor.column > self.max_column() || coor.row > self.max_row() {
            return None;
        }
        Some(&mut self.grid[coor.row][coor.column])
    }

    pub fn max_row(&self) -> usize {
        self.grid.len() - 1
    }

    pub fn max_column(&self) -> usize {
        self.grid[0].len() - 1
    }

    pub fn get_surrounding(&self, coor: &Coordinate) -> Vec<(Coordinate, &T)> {
        self.planner
            .get_surrounding(coor)
            .iter()
            .map(|x| (x.clone(), &self.grid[x.row][x.column]))
            .collect::<Vec<_>>()
    }
    pub fn get_neighbors(&self, coor: &Coordinate) -> Vec<(Coordinate, &T)> {
        self.planner
            .get_neighbors(coor)
            .iter()
            .map(|x| (x.clone(), &self.grid[x.row][x.column]))
            .collect::<Vec<_>>()
    }

    pub fn march(&self, coor: &Coordinate, direction: Direction) -> Vec<(Coordinate, &T)> {
        self.planner
            .cast_ray(coor, direction)
            .iter()
            .map(|x| (x.clone(), &self.grid[x.row][x.column]))
            .collect::<Vec<_>>()
    }
}

impl<T: Clone> Grid<T> {
    pub fn uniform(input: T, rows: usize, columns: usize) -> Self {
        Self {
            grid: vec![vec![input; columns]; rows],
            planner: GridPlanner::new(rows, columns),
        }
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Grid<T> {
    type Error = &'static str;

    fn try_from(input: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        Self::new(input)
    }
}
pub struct GridBorrow<'a, T> {
    grid: &'a Grid<T>,
    row: usize,
    col: usize,
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = (Coordinate, &'a T);
    type IntoIter = GridBorrow<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        GridBorrow {
            grid: self,
            row: 0,
            col: 0,
        }
    }
}

impl<'a, T> Iterator for GridBorrow<'a, T> {
    type Item = (Coordinate, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row > self.grid.max_row() {
            return None;
        }
        let result = &self.grid.grid[self.row][self.col];
        let coor = (self.row, self.col);
        self.col += 1;
        if self.col > self.grid.max_column() {
            self.row += 1;
            self.col = 0;
        }
        Some((coor.into(), result))
    }
}
impl<T: std::fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let payload = self
            .grid
            .iter()
            .map(|row| row.iter().map(|x| x.to_string()).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", payload)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct Tile(usize);

    #[test]
    fn test_grid_iter() {
        let grid: Grid<Tile> = vec![vec![Tile(0), Tile(1)], vec![Tile(2), Tile(3)]]
            .try_into()
            .unwrap();
        let items: Vec<_> = grid.into_iter().collect();

        assert_eq!(items[0], ((0, 0).into(), &Tile(0)));
        assert_eq!(items[1], ((0, 1).into(), &Tile(1)));
        assert_eq!(items[2], ((1, 0).into(), &Tile(2)));
        assert_eq!(items[3], ((1, 1).into(), &Tile(3)));
    }
    #[test]
    fn test_grid_get() {
        let grid: Grid<usize> = vec![vec![0, 1], vec![2, 3]].try_into().unwrap();
        let two = grid.get(&(1, 0).into());

        assert_eq!(*two.unwrap(), 2);

        let not_exist = grid.get(&(9, 9).into());
        assert!(not_exist.is_none())
    }
    #[test]
    fn test_grid_get_surounding() {
        let grid: Grid<usize> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
            .try_into()
            .unwrap();
        assert_eq!(grid.get_surrounding(&(0, 1).into()).len(), 5);
        assert_eq!(grid.get_surrounding(&(0, 0).into()).len(), 3);
        assert_eq!(grid.get_surrounding(&(1, 1).into()).len(), 8);
    }

    #[test]
    fn test_grid_get_neighbors() {
        let grid: Grid<usize> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
            .try_into()
            .unwrap();
        assert_eq!(grid.get_neighbors(&(0, 1).into()).len(), 3);
        assert_eq!(grid.get_neighbors(&(0, 0).into()).len(), 2);
        assert_eq!(grid.get_neighbors(&(1, 1).into()).len(), 4);
    }

    #[test]
    fn test_grid_march() {
        let grid: Grid<usize> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
            .try_into()
            .unwrap();
        assert_eq!(grid.march(&(0, 0).into(), Direction::LowerRight).len(), 3);
        assert_eq!(grid.march(&(0, 0).into(), Direction::Left).len(), 1);
    }
}
