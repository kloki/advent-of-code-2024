use super::models::Coordinate;

#[derive(Clone, Debug)]
pub enum Direction {
    UpperLeft,
    Upper,
    UpperRight,
    Left,
    Right,
    LowerLeft,
    Lower,
    LowerRight,
}

impl Direction {
    pub fn all() -> [Direction; 8] {
        [
            Direction::UpperLeft,
            Direction::Upper,
            Direction::UpperRight,
            Direction::Left,
            Direction::Right,
            Direction::LowerLeft,
            Direction::Lower,
            Direction::LowerRight,
        ]
    }
}

#[derive(Clone, Debug)]
pub struct GridPlanner {
    rows: usize,
    columns: usize,
}

impl GridPlanner {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self { rows, columns }
    }

    pub fn get_upper(&self, coor: &Coordinate) -> Option<Coordinate> {
        if coor.row == 0 {
            None
        } else {
            Some(Coordinate {
                row: coor.row - 1,
                column: coor.column,
            })
        }
    }

    pub fn get_lower(&self, coor: &Coordinate) -> Option<Coordinate> {
        if coor.row == self.rows - 1 {
            None
        } else {
            Some(Coordinate {
                row: coor.row + 1,
                column: coor.column,
            })
        }
    }

    pub fn get_left(&self, coor: &Coordinate) -> Option<Coordinate> {
        if coor.column == 0 {
            None
        } else {
            Some(Coordinate {
                row: coor.row,
                column: coor.column - 1,
            })
        }
    }

    pub fn get_right(&self, coor: &Coordinate) -> Option<Coordinate> {
        if coor.column == self.columns - 1 {
            None
        } else {
            Some(Coordinate {
                row: coor.row,
                column: coor.column + 1,
            })
        }
    }

    pub fn get_upper_left(&self, coor: &Coordinate) -> Option<Coordinate> {
        if coor.row == 0 || coor.column == 0 {
            None
        } else {
            Some(Coordinate {
                row: coor.row - 1,
                column: coor.column - 1,
            })
        }
    }

    pub fn get_upper_right(&self, coor: &Coordinate) -> Option<Coordinate> {
        if coor.row == 0 || coor.column == self.columns - 1 {
            None
        } else {
            Some(Coordinate {
                row: coor.row - 1,
                column: coor.column + 1,
            })
        }
    }

    pub fn get_lower_left(&self, coor: &Coordinate) -> Option<Coordinate> {
        if coor.row == self.rows - 1 || coor.column == 0 {
            None
        } else {
            Some(Coordinate {
                row: coor.row + 1,
                column: coor.column - 1,
            })
        }
    }

    pub fn get_lower_right(&self, coor: &Coordinate) -> Option<Coordinate> {
        if coor.row == self.rows - 1 || coor.column == self.columns - 1 {
            None
        } else {
            Some(Coordinate {
                row: coor.row + 1,
                column: coor.column + 1,
            })
        }
    }

    pub fn get_direction(&self, coor: &Coordinate, direction: &Direction) -> Option<Coordinate> {
        match direction {
            Direction::UpperLeft => self.get_upper_left(coor),
            Direction::Upper => self.get_upper(coor),
            Direction::UpperRight => self.get_upper_right(coor),
            Direction::Left => self.get_left(coor),
            Direction::Right => self.get_right(coor),
            Direction::LowerLeft => self.get_lower_left(coor),
            Direction::Lower => self.get_lower(coor),
            Direction::LowerRight => self.get_lower_right(coor),
        }
    }

    pub fn get_surrounding(&self, coor: &Coordinate) -> Vec<Coordinate> {
        vec![
            self.get_upper_left(coor),
            self.get_upper(coor),
            self.get_upper_right(coor),
            self.get_left(coor),
            self.get_right(coor),
            self.get_lower_left(coor),
            self.get_lower(coor),
            self.get_lower_right(coor),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    pub fn get_neighbors(&self, coor: &Coordinate) -> Vec<Coordinate> {
        vec![
            self.get_upper(coor),
            self.get_left(coor),
            self.get_right(coor),
            self.get_lower(coor),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    pub fn cast_ray(&self, coor: &Coordinate, direction: Direction) -> Vec<Coordinate> {
        let mut result = vec![coor.clone()];
        loop {
            match self.get_direction(result.last().unwrap(), &direction) {
                Some(new_coor) => result.push(new_coor),
                None => break,
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_upper() {
        let planner = GridPlanner::new(5, 5);
        assert_eq!(
            planner.get_upper(&Coordinate { row: 1, column: 0 }),
            Some(Coordinate { row: 0, column: 0 })
        );
        assert_eq!(planner.get_upper(&Coordinate { row: 0, column: 0 }), None);
    }

    #[test]
    fn test_get_lower() {
        let planner = GridPlanner::new(5, 5);
        assert_eq!(
            planner.get_lower(&Coordinate { row: 3, column: 0 }),
            Some(Coordinate { row: 4, column: 0 })
        );
        assert_eq!(planner.get_lower(&Coordinate { row: 4, column: 0 }), None);
    }

    #[test]
    fn test_get_left() {
        let planner = GridPlanner::new(5, 5);
        assert_eq!(
            planner.get_left(&Coordinate { row: 0, column: 1 }),
            Some(Coordinate { row: 0, column: 0 })
        );
        assert_eq!(planner.get_left(&Coordinate { row: 0, column: 0 }), None);
    }

    #[test]
    fn test_get_right() {
        let planner = GridPlanner::new(5, 5);
        assert_eq!(
            planner.get_right(&Coordinate { row: 0, column: 3 }),
            Some(Coordinate { row: 0, column: 4 })
        );
        assert_eq!(planner.get_right(&Coordinate { row: 0, column: 4 }), None);
    }

    #[test]
    fn test_get_upper_left() {
        let planner = GridPlanner::new(5, 5);
        assert_eq!(
            planner.get_upper_left(&Coordinate { row: 1, column: 1 }),
            Some(Coordinate { row: 0, column: 0 })
        );
        assert_eq!(
            planner.get_upper_left(&Coordinate { row: 0, column: 0 }),
            None
        );
        assert_eq!(
            planner.get_upper_left(&Coordinate { row: 1, column: 0 }),
            None
        );
    }

    #[test]
    fn test_get_upper_right() {
        let planner = GridPlanner::new(5, 5);
        assert_eq!(
            planner.get_upper_right(&Coordinate { row: 1, column: 1 }),
            Some(Coordinate { row: 0, column: 2 })
        );
        assert_eq!(
            planner.get_upper_right(&Coordinate { row: 0, column: 4 }),
            None
        );
        assert_eq!(
            planner.get_upper_right(&Coordinate { row: 0, column: 1 }),
            None
        );
    }

    #[test]
    fn test_get_lower_left() {
        let planner = GridPlanner::new(5, 5);
        assert_eq!(
            planner.get_lower_left(&Coordinate { row: 3, column: 1 }),
            Some(Coordinate { row: 4, column: 0 })
        );
        assert_eq!(
            planner.get_lower_left(&Coordinate { row: 4, column: 0 }),
            None
        );
        assert_eq!(
            planner.get_lower_left(&Coordinate { row: 1, column: 0 }),
            None
        );
    }

    #[test]
    fn test_get_lower_right() {
        let planner = GridPlanner::new(5, 5);
        assert_eq!(
            planner.get_lower_right(&Coordinate { row: 3, column: 1 }),
            Some(Coordinate { row: 4, column: 2 })
        );
        assert_eq!(
            planner.get_lower_right(&Coordinate { row: 4, column: 4 }),
            None
        );
        assert_eq!(
            planner.get_lower_right(&Coordinate { row: 1, column: 4 }),
            None
        );
    }
}
