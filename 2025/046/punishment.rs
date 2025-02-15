// https://leetcode.com/problems/find-the-punishment-number-of-an-integer/

pub fn punishment_number(n: i32) -> i32 {
    fn can_partition(mut num: i32, target: i32, sum: i32) -> bool {
        if sum > target {
            return false;
        }
        if num == 0 {
            return sum == target;
        }

        let mut power = 1;
        let mut current = 0;

        while num > 0 {
            current = (num % 10) * power + current;
            num /= 10;
            power *= 10;

            if can_partition(num, target, sum + current) {
                return true;
            }
        }
        false
    }

    (1..=n)
        .filter(|&i| can_partition(i * i, i, 0))
        .map(|i| i * i)
        .sum()
}
