// https://leetcode.com/problems/minimum-time-to-repair-cars/submissions/1575690541/

pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
    pub fn is_repaired(ranks: &Vec<i32>, cars: i64, time: i64) -> bool {
        let mut cars_repaired = 0;
        for &rank in ranks {
            // r * n * n = time
            // n = sqrt (time / r)
            cars_repaired += (time / rank as i64).isqrt();
        }
        cars_repaired >= cars
    }

    let mut low = 0;
    let cars = cars as i64;
    let mut high = cars * cars * *ranks.iter().max().unwrap() as i64;

    while low < high {
        let mid = (low + high) / 2;
        if is_repaired(&ranks, cars, mid) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    low
}
