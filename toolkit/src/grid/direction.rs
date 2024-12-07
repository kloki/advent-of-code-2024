#[derive(Clone, Debug)]
pub enum Direction {
    UpLeft,
    Up,
    UpRight,
    Left,
    Right,
    DownLeft,
    Down,
    DownRight,
}

impl Direction {
    pub fn all() -> [Direction; 8] {
        [
            Direction::UpLeft,
            Direction::Up,
            Direction::UpRight,
            Direction::Left,
            Direction::Right,
            Direction::DownLeft,
            Direction::Down,
            Direction::DownRight,
        ]
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::UpLeft => Direction::UpRight,
            Direction::Up => Direction::Right,
            Direction::UpRight => Direction::DownRight,
            Direction::Right => Direction::Down,
            Direction::DownRight => Direction::DownLeft,
            Direction::Down => Direction::Left,
            Direction::DownLeft => Direction::UpLeft,
            Direction::Left => Direction::Up,
        }
    }

    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::UpLeft => Direction::DownLeft,
            Direction::Left => Direction::Down,
            Direction::DownLeft => Direction::DownRight,
            Direction::Down => Direction::Right,
            Direction::DownRight => Direction::UpRight,
            Direction::Right => Direction::Up,
            Direction::UpRight => Direction::UpLeft,
            Direction::Up => Direction::Left,
        }
    }
}
