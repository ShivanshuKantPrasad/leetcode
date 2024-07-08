// https://leetcode.com/problems/find-the-winner-of-the-circular-game/editorial/

pub fn find_the_winner(n: i32, k: i32) -> i32 {
    let mut friends = (1..n + 1).collect::<Vec<_>>();
    let mut cur = 0;
    while friends.len() > 1 {
        let x = (cur + k - 1) as usize % friends.len();
        let y = friends.remove(x);
        cur = x as i32;
    }
    friends[0]
}
