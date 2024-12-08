use std::{collections::HashMap, fmt};

use toolkit::{
    grid::{printer::GridPrinter, Coordinate, Grid},
    input::get_input,
};

#[derive(Clone, Copy, Debug)]
struct Tile {
    has_anti_node: bool,
    frequency: Option<char>,
}

impl Tile {
    fn new(frequency: char) -> Self {
        let frequency = match frequency {
            '.' => None,
            c => Some(c),
        };
        Self {
            frequency,
            has_anti_node: false,
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.frequency.unwrap_or('.'))
    }
}

fn parse_input(input: String) -> Grid<Tile> {
    Grid::new(
        input
            .trim()
            .split("\n")
            .map(|line| line.chars().map(Tile::new).collect::<Vec<_>>())
            .collect(),
    )
    .unwrap()
}
enum Harmonics {
    Double,
    Every,
}

impl Harmonics {
    fn find_anti_nodes(
        &self,
        input: &[Coordinate],
        max_row: usize,
        max_column: usize,
    ) -> Vec<Coordinate> {
        match self {
            Harmonics::Double => find_anti_nodes_double(input),
            Harmonics::Every => find_anti_nodes_every(input, max_row, max_column),
        }
    }
}

fn find_anti_nodes_double(input: &[Coordinate]) -> Vec<Coordinate> {
    let mut nodes = input.to_vec();
    let mut result = Vec::new();
    while let Some(node) = nodes.pop() {
        for other in nodes.clone().into_iter() {
            let distance = node - other;
            if let Ok(new_node) = node + distance {
                result.push(new_node)
            }
            if let Ok(new_node) = other - distance {
                result.push(new_node)
            }
        }
    }

    result
}

fn find_anti_nodes_every(
    input: &[Coordinate],
    max_row: usize,
    max_column: usize,
) -> Vec<Coordinate> {
    let mut nodes = input.to_vec();
    let mut result = Vec::new();
    while let Some(node) = nodes.pop() {
        for other in nodes.clone().into_iter() {
            let distance = node - other;
            let mut current = node;
            while let Ok(new_node) = current - distance {
                if !new_node.in_bounds(max_row, max_column) {
                    break;
                }
                current = new_node;
                result.push(new_node)
            }
            while let Ok(new_node) = current + distance {
                if !new_node.in_bounds(max_row, max_column) {
                    break;
                }
                current = new_node;
                result.push(new_node)
            }
        }
    }

    result
}
fn process(grid: &mut Grid<Tile>, harmonics: Harmonics) {
    let mut freqencies_sets: HashMap<char, Vec<Coordinate>> = HashMap::new();
    for (coor, tile) in grid.into_iter() {
        if let Some(f) = tile.frequency {
            let entry = freqencies_sets.entry(f).or_default();
            entry.push(coor);
        }
    }

    let mut found_nodes: Vec<Coordinate> = Vec::new();

    for frequency_set in freqencies_sets.values() {
        found_nodes.append(&mut harmonics.find_anti_nodes(
            frequency_set,
            grid.max_row(),
            grid.max_column(),
        ));
    }
    let mut pp = GridPrinter::new(grid.clone());

    for node in found_nodes {
        if let Some(tile) = grid.get_mut(&node) {
            tile.has_anti_node = true;
            pp = pp.mark_green(node);
        }
    }
    println!("{}", pp.print());
}

fn main() {
    let contents = get_input();
    let mut grid = parse_input(contents);
    let mut grid_1 = grid.clone();
    process(&mut grid_1, Harmonics::Double);

    println!(
        "Solution 1 {}",
        grid_1.into_iter().filter(|(_, x)| x.has_anti_node).count()
    );

    process(&mut grid, Harmonics::Every);

    println!(
        "Solution 2 {}",
        grid.into_iter().filter(|(_, x)| x.has_anti_node).count()
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
    #[test]
    fn test_6_anti_nodes_double() {
        let mut grid = parse_input(TEST_INPUT.to_string());
        process(&mut grid, Harmonics::Double);
        assert_eq!(
            grid.into_iter().filter(|(_, x)| x.has_anti_node).count(),
            14
        )
    }

    #[test]
    fn test_6_anti_nodes_every() {
        let mut grid = parse_input(TEST_INPUT.to_string());
        process(&mut grid, Harmonics::Every);
        assert_eq!(
            grid.into_iter().filter(|(_, x)| x.has_anti_node).count(),
            34
        )
    }
}
