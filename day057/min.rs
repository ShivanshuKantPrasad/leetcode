// https://leetcode.com/problems/minimum-number-of-days-to-make-m-bouquets/

pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
    if m * k > bloom_day.len() as i32 {
        return -1;
    }

    let mut low = 0;
    let mut high = 1000_000_010;
    while low <= high {
        let cur = (low + high) / 2;
        let mut bloom = 0;
        let mut bouquets_made = 0;
        for i in 0..bloom_day.len() {
            if bloom_day[i] <= cur {
                bloom += 1;
            } else {
                bouquets_made += bloom / k;
                bloom = 0;
            }
        }
        bouquets_made += bloom / k;
        if bouquets_made >= m {
            high = cur - 1;
        } else {
            low = cur + 1;
        }
    }

    if low >= 1000_000_010 {
        -1
    } else {
        low
    }
}

fn main() {
    println!("{}", min_days(vec![1, 10, 3, 10, 2], 3, 1));
    println!("{}", min_days(vec![1, 10, 3, 10, 2], 3, 2));
    println!("{}", min_days(vec![7, 7, 7, 7, 12, 7, 7], 2, 3));
    println!("{}", min_days(vec![1000000000, 1000000000], 1, 1));
    println!("{}", min_days(vec![1, 10, 2, 9, 3, 8, 4, 7, 5, 6], 4, 2));
}
