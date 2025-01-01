// https://leetcode.com/problems/move-pieces-to-obtain-a-string/description/

pub fn can_change(start: String, target: String) -> bool {
    let start = start.as_bytes();
    let target = target.as_bytes();
    let n1 = start.len();

    let mut i = 0;
    let mut j = 0;

    while i < n1 || j < n1 {
        while i < n1 && start[i] == b'_' {
            i += 1;
        }

        while j < n1 && target[j] == b'_' {
            j += 1;
        }

        if i == n1 || j == n1 {
            return i == n1 && j == n1;
        }

        if start[i] != target[j] || (start[i] == b'L' && i < j) || (start[i] == b'R' && i > j) {
            return false;
        }
        i += 1;
        j += 1;
    }
    true
}
