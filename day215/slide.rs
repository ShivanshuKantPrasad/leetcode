// https://leetcode.com/problems/sliding-puzzle/

pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
    use std::collections::{HashSet, VecDeque};

    let directions = vec![
        vec![1, 3],
        vec![0, 2, 4],
        vec![1, 5],
        vec![0, 4],
        vec![1, 3, 5],
        vec![2, 4],
    ];

    let target = "123450";
    let mut start_state = board
        .iter()
        .flatten()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    if start_state == target {
        return 0;
    }

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start_state.clone(), 0));
    visited.insert(start_state.clone());

    while let Some((current_state, moves)) = queue.pop_front() {
        let zero_pos = current_state.find('0').unwrap();

        for &new_pos in &directions[zero_pos] {
            let mut temp = current_state.clone().into_bytes();
            temp.swap(zero_pos, new_pos);
            let next_state = String::from_utf8(temp.to_vec()).unwrap();

            if next_state == target {
                return moves + 1;
            }

            if visited.contains(&next_state) {
                continue;
            }

            visited.insert(next_state.clone());
            queue.push_back((next_state, moves + 1));
        }
    }
    -1
}
