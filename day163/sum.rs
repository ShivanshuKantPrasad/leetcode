// https://leetcode.com/problems/divide-players-into-teams-of-equal-skill/

pub fn divide_players(mut skill: Vec<i32>) -> i64 {
    let n = skill.len() / 2;
    skill.sort_unstable();

    let mut l = 0;
    let mut r = skill.len() - 1;
    let mut ans = 0;
    let sum = skill[l] + skill[r];

    while l < r {
        if skill[l] + skill[r] != sum {
            return -1;
        }
        ans += (skill[l] * skill[r]) as i64;

        l += 1;
        r -= 1;
    }

    ans
}
