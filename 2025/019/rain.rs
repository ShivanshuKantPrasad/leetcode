#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Cell {
    height: i32,
    row: usize,
    col: usize,
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.height.cmp(&self.height)
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
    let d_row = [0, 0, -1, 1];
    let d_col = [-1, 1, 0, 0];

    let num_rows = height_map.len();
    let num_cols = height_map[0].len();

    let mut visited = vec![vec![false; num_cols]; num_rows];
    let mut boundary = std::collections::BinaryHeap::new();

    for i in 0..num_rows {
        boundary.push(Cell {
            height: height_map[i][0],
            row: i,
            col: 0,
        });
        boundary.push(Cell {
            height: height_map[i][num_cols - 1],
            row: i,
            col: num_cols - 1,
        });
        visited[i][0] = true;
        visited[i][num_cols - 1] = true;
    }

    for j in 0..num_cols {
        boundary.push(Cell {
            height: height_map[0][j],
            row: 0,
            col: j,
        });
        boundary.push(Cell {
            height: height_map[num_rows - 1][j],
            row: num_rows - 1,
            col: j,
        });
        visited[0][j] = true;
        visited[num_rows - 1][j] = true;
    }

    let mut total_water_volume = 0;

    while let Some(current_cell) = boundary.pop() {
        let current_row = current_cell.row;
        let current_col = current_cell.col;
        let min_boundary_height = current_cell.height;

        for direction in 0..4 {
            let neighbor_row = (current_row as isize + d_row[direction]) as usize;
            let neighbor_col = (current_col as isize + d_col[direction]) as usize;

            if is_valid_cell(neighbor_row, neighbor_col, num_rows, num_cols)
                && !visited[neighbor_row][neighbor_col]
            {
                let neighbor_height = height_map[neighbor_row][neighbor_col];

                if neighbor_height < min_boundary_height {
                    total_water_volume += min_boundary_height - neighbor_height;
                }

                boundary.push(Cell {
                    height: neighbor_height.max(min_boundary_height),
                    row: neighbor_row,
                    col: neighbor_col,
                });
                visited[neighbor_row][neighbor_col] = true;
            }
        }
    }

    fn is_valid_cell(row: usize, col: usize, num_rows: usize, num_cols: usize) -> bool {
        row < num_rows && col < num_cols
    }

    total_water_volume
}
