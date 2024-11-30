pub fn surrounding_coor(
    row: usize,
    column: usize,
    max_row: usize,
    max_column: usize,
) -> Vec<(usize, usize)> {
    let mut coor: Vec<(usize, usize)> = Vec::new();
    if row > 0 {
        if column > 0 {
            coor.push((row - 1, column - 1));
        }
        coor.push((row - 1, column));
        if column < max_column {
            coor.push((row - 1, column + 1));
        }
    }
    if column > 0 {
        coor.push((row, column - 1));
    }

    if column < max_column {
        coor.push((row, column + 1));
    }

    if row < max_row {
        if column > 0 {
            coor.push((row + 1, column - 1));
        }
        coor.push((row + 1, column));
        if column < max_column {
            coor.push((row + 1, column + 1));
        }
    }

    coor
}

pub fn neighbor_coor(
    row: usize,
    column: usize,
    max_row: usize,
    max_column: usize,
) -> Vec<(usize, usize)> {
    let mut coor: Vec<(usize, usize)> = Vec::new();
    if row > 0 {
        coor.push((row - 1, column));
    }
    if column > 0 {
        coor.push((row, column - 1));
    }

    if row < max_column {
        coor.push((row, column + 1));
    }

    if row < max_row {
        coor.push((row + 1, column));
    }

    coor
}
