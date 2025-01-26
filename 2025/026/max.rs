// https://leetcode.com/problems/maximum-employees-to-be-invited-to-a-meeting/submissions/1521072383/

pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
    let n = favorite.len();
    let mut in_degree = vec![0; n];

    for person in 0..n {
        in_degree[favorite[person] as usize] += 1;
    }

    use std::collections::VecDeque;
    let mut q = VecDeque::new();

    for person in 0..n {
        if in_degree[person] == 0 {
            q.push_back(person);
        }
    }

    let mut depth = vec![1; n];
    while let Some(current_node) = q.pop_front() {
        let next_node = favorite[current_node] as usize;
        depth[next_node] = depth[next_node].max(depth[current_node] + 1);
        in_degree[next_node] -= 1;
        if in_degree[next_node] == 0 {
            q.push_back(next_node);
        }
    }

    let mut longest_cycle = 0;
    let mut two_cycle_invitations = 0;

    for person in 0..n {
        if in_degree[person] == 0 {
            continue;
        }

        let mut cycle_length = 0;
        let mut current = person;

        while in_degree[current] != 0 {
            in_degree[current] = 0;
            cycle_length += 1;
            current = favorite[current] as usize;
        }

        if cycle_length == 2 {
            two_cycle_invitations += depth[person] + depth[favorite[person] as usize];
        } else {
            longest_cycle = longest_cycle.max(cycle_length);
        }
    }

    longest_cycle.max(two_cycle_invitations)
}
