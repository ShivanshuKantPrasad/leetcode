struct Solution;
impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![Vec::new(); grid.len() - 2];

        for i in 0..grid.len() - 2 {
            for j in 0..grid[0].len() - 2 {
                let mut max = i32::MIN;

                for k in 0..3 {
                    for h in 0..3 {
                        max = i32::max(max, grid[i + k][j + h]);
                    }
                }

                res[i].push(max);
            }
        }

        res
    }
}

fn main() {
    println!("{:?}",
        Solution::largest_local(
            vec![vec![9,9,8,1],vec![5,6,2,6],vec![8,2,6,4],vec![6,2,2,2]]
        )
    );

    println!("{:?}",
        Solution::largest_local(
            vec![vec![1,1,1,1,1],vec![1,1,1,1,1],vec![1,1,2,1,1],vec![1,1,1,1,1],vec![1,1,1,1,1]]
        )
    );
}
