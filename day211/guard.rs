// https://leetcode.com/problems/count-unguarded-cells-in-the-grid/description/

pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
    #[derive(Clone, PartialEq, Eq, Debug)]
    enum State {
        Guard,
        Guarded,
        Wall,
        Unguarded,
    }

    let (m, n) = (m as usize, n as usize);
    let mut cells = vec![vec![State::Unguarded; n]; m];

    for wall in walls {
        cells[wall[0] as usize][wall[1] as usize] = State::Wall;
    }

    let guards = guards
        .into_iter()
        .map(|guard| (guard[0] as usize, guard[1] as usize))
        .collect::<Vec<(_, _)>>();

    for &(row, col) in guards.iter() {
        cells[row][col] = State::Guard;
    }

    for (row, col) in guards {
        for i in (0..row).rev() {
            match cells[i][col] {
                State::Guard | State::Wall => break,
                _ => cells[i][col] = State::Guarded,
            }
        }

        for i in (row + 1)..m {
            match cells[i][col] {
                State::Guard | State::Wall => break,
                _ => cells[i][col] = State::Guarded,
            }
        }

        for j in (0..col).rev() {
            match cells[row][j] {
                State::Guard | State::Wall => break,
                _ => cells[row][j] = State::Guarded,
            }
        }

        for j in (col + 1)..n {
            match cells[row][j] {
                State::Guard | State::Wall => break,
                _ => cells[row][j] = State::Guarded,
            }
        }
    }

    cells
        .into_iter()
        .map(|row| {
            row.into_iter()
                .filter(|cell| *cell == State::Unguarded)
                .count() as i32
        })
        .sum()
}
