// https://leetcode.com/problems/minimum-total-distance-traveled/

pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
    robot.sort_unstable();
    factory.sort_unstable();

    let factory_pos = factory
        .iter()
        .flat_map(|f| vec![f[0] as i64; f[1] as usize])
        .collect::<Vec<_>>();
    let robot_count = robot.len();
    let factory_count = factory_pos.len();

    let mut dp = vec![vec![0; factory_count + 1]; robot_count + 1];

    for i in 0..robot_count {
        dp[i][factory_count] = 1e12 as i64;
    }

    for i in (0..robot_count).rev() {
        for j in (0..factory_count).rev() {
            let assign = (robot[i] as i64 - factory_pos[j]).abs() + dp[i + 1][j + 1];
            let skip = dp[i][j + 1];
            dp[i][j] = assign.min(skip);
        }
    }

    dp[0][0]
}
