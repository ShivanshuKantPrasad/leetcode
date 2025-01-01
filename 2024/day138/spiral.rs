//https://leetcode.com/problems/spiral-matrix-iv/
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn spiral_matrix(m: i32, n: i32, mut head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
    let mut mat = vec![vec![-1; n as usize]; m as usize];
    let mut row = 0;
    let mut col = 0;
    let directions = [[0, 1], [1, 0], [0, -1], [-1, 0]];
    let mut cur_dir = 0;

    while let Some(node) = head {
        mat[row as usize][col as usize] = node.val;

        cur_dir = match cur_dir {
            0 if col + row == n - 1 => 1,
            1 if row + n - col == m => 2,
            2 if col == (m - row - 1) => 3,
            3 if row == (col + 1) => 0,
            _ => cur_dir,
        };

        row += directions[cur_dir][0];
        col += directions[cur_dir][1];
        head = node.next;
    }

    mat
}
