use std::fmt;

use toolkit::{
    grid::{Coordinate, Direction, Grid},
    input::get_input,
};

#[derive(Debug, PartialEq, Eq, Clone)]
enum TileType {
    Open,
    Start,
    Obstacle,
}

#[derive(Debug, Eq, Clone)]
struct Tile {
    visited: bool,
    pub tile_type: TileType,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let char = match self.tile_type {
            TileType::Open => '_',
            TileType::Obstacle => '#',
            TileType::Start => '^',
        };
        write!(f, "{}", char)
    }
}
impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        Self {
            visited: false,
            tile_type,
        }
    }

    pub fn mark_visited(&mut self) {
        self.visited = true
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.tile_type == other.tile_type
    }
}

fn parse_input(input: String) -> (Coordinate, Grid<Tile>) {
    let grid: Grid<Tile> = Grid::new(
        input
            .trim()
            .split("\n")
            .map(|line| {
                line.chars()
                    .map(|x| match x {
                        '#' => Tile::new(TileType::Obstacle),
                        '^' => Tile::new(TileType::Start),
                        _ => Tile::new(TileType::Open),
                    })
                    .collect::<Vec<_>>()
            })
            .collect(),
    )
    .unwrap();
    let start = grid
        .into_iter()
        .filter(|(_, tile)| tile.tile_type == TileType::Start)
        .map(|(coor, _)| coor)
        .next()
        .unwrap();
    (start, grid)
}

fn walk(start: Coordinate, grid: &mut Grid<Tile>) -> bool {
    let mut current = start;
    let mut posistions = vec![start];
    let mut direction = Direction::Up;
    let wall = Tile::new(TileType::Obstacle);

    loop {
        let (path, collided) = grid.march_until(&current, direction.clone(), &wall);
        let coordinates: Vec<Coordinate> = path.iter().map(|(coor, _)| *coor).collect();
        for coor in &coordinates {
            grid.get_mut(coor).unwrap().mark_visited();
        }

        current = *coordinates.last().unwrap();
        if posistions.contains(&current) {
            // if we haven moved it doens't count
            if coordinates.len() != 1 {
                return false;
            }
        } else {
            posistions.push(current);
        }

        direction = direction.turn_right();
        if !collided {
            return true;
        }
    }
}

fn main() {
    let contents = get_input();
    let (start, grid) = parse_input(contents);
    let mut grid_first_round = grid.clone();
    walk(start, &mut grid_first_round);
    println!(
        "Solution 1 {}",
        grid_first_round
            .into_iter()
            .filter(|(_, x)| x.visited)
            .count()
    );

    let candidates: Vec<Coordinate> = grid_first_round
        .into_iter()
        .filter(|(coordinate, tile)| tile.visited && *coordinate != start)
        .map(|(coor, _)| coor)
        .collect();

    let mut score = 0;

    for candidate in candidates {
        let mut new_grid = grid.clone();
        new_grid.get_mut(&candidate).unwrap().tile_type = TileType::Obstacle;
        if !walk(start, &mut new_grid) {
            score += 1;
        }
    }

    println!(
        "Solution 2 {}, for my test input this score was of by one!",
        score
    );
}

#[cfg(test)]
mod tests {
    use toolkit::grid::printer::GridPrinter;

    use super::*;
    const TEST_INPUT: &str = " ....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
    #[test]
    fn test_6_find_path() {
        let (start, mut grid) = parse_input(TEST_INPUT.to_string());
        assert!(walk(start, &mut grid));
        assert_eq!(grid.into_iter().filter(|(_, x)| x.visited).count(), 41)
    }

    #[test]
    fn test_6_find_obsticale() {
        let (start, grid) = parse_input(TEST_INPUT.to_string());
        let mut grid_first_round = grid.clone();
        walk(start, &mut grid_first_round);
        let candidates: Vec<Coordinate> = grid_first_round
            .into_iter()
            .filter(|(coordinate, tile)| tile.visited && *coordinate != start)
            .map(|(coor, _)| coor)
            .collect();

        let mut score = 0;

        let mut pp = GridPrinter::new(grid.clone());
        for c in &candidates {
            pp = pp.mark_green(*c);
        }

        for candidate in candidates {
            let mut new_grid = grid.clone();
            new_grid.get_mut(&candidate).unwrap().tile_type = TileType::Obstacle;
            if !walk(start, &mut new_grid) {
                score += 1;
                pp = pp.mark_red(candidate);
            }
        }
        println!("{}", pp.print());
        assert_eq!(score, 6)
    }
}
