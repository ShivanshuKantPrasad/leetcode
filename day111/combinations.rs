// https://leetcode.com/problems/combination-sum-ii/

pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn backtrack(
        candidates: &Vec<i32>,
        target: i32,
        cur_index: usize,
        path: &mut Vec<i32>,
        answer: &mut Vec<Vec<i32>>,
    ) {
        if target < 0 {
            return;
        }
        if target == 0 {
            answer.push(path.clone());
            return;
        }

        for i in cur_index..candidates.len() {
            if i > cur_index && candidates[i] == candidates[i - 1] {
                continue;
            }
            path.push(candidates[i]);
            backtrack(candidates, target - candidates[i], i + 1, path, answer);
            path.pop();
        }
    }
    let mut result = Vec::new();
    candidates.sort_unstable();
    backtrack(&candidates, target, 0, &mut Vec::new(), &mut result);
    result
}
