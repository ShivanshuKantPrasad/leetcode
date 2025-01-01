// https://leetcode.com/problems/robot-collisions/

fn survived_robots_healths(positions: Vec<i32>, healths: Vec<i32>, directions: String) -> Vec<i32> {
    let directions = directions.as_bytes();
    let len = positions.len();
    let mut robots = (0..len)
        .map(|i| (positions[i], healths[i], directions[i], i))
        .collect::<Vec<_>>();
    robots.sort_unstable_by_key(|robot| robot.0);

    let mut stack = Vec::new();
    stack.push(robots[0]);

    for i in 1..positions.len() {
        let mut cur = robots[i];
        if cur.2 == b'R' {
            stack.push(cur);
            continue;
        }
        while let Some(mut prev) = stack.pop() {
            if cur.1 == 0 {
                stack.push(prev);
                break;
            }
            if prev.2 == b'R' {
                if prev.1 > cur.1 {
                    cur.1 = 0;
                    prev.1 -= 1;
                    stack.push(prev);
                } else if prev.1 == cur.1 {
                    cur.1 = 0;
                } else {
                    cur.1 -= 1;
                }
            } else {
                stack.push(prev);
                break;
            }
        }
        if cur.1 != 0 {
            stack.push(cur);
        }
    }

    stack.sort_unstable_by_key(|robot| robot.3);
    stack.iter().map(|robot| robot.1).collect()
}

fn main() {
    println!(
        "{:?}",
        survived_robots_healths(
            vec![5, 4, 3, 2, 1],
            vec![2, 17, 9, 15, 10],
            "RRRRR".to_string()
        )
    );
    println!(
        "{:?}",
        survived_robots_healths(vec![3, 5, 2, 6], vec![10, 10, 15, 12], "RLRL".to_string())
    );
    println!(
        "{:?}",
        survived_robots_healths(vec![1, 2, 5, 6], vec![10, 10, 11, 11], "RLRL".to_string())
    );
    println!(
        "{:?}",
        survived_robots_healths(vec![11, 44, 16], vec![1, 20, 17], "RLR".to_string())
    );
    println!(
        "{:?}",
        survived_robots_healths(
            vec![29, 7, 31, 42],
            vec![19, 24, 26, 10],
            "RRLR".to_string()
        )
    );
    println!(
        "{:?}",
        survived_robots_healths(vec![3, 40], vec![49, 11], "LL".to_string())
    );
}
