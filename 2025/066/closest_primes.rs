// https://leetcode.com/problems/closest-prime-numbers-in-range/description/

pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
    let mut primes = vec![true; right as usize + 2];
    let n = primes.len();
    primes[0] = false;
    primes[1] = false;

    let mut i = 2;

    while i < n {
        for x in ((2 * i)..n).step_by(i) {
            primes[x] = false;
        }

        i += 1;
        while i < n && primes[i] == false {
            i += 1;
        }
    }

    let mut smallest_gap = usize::MAX;
    let mut res = vec![-1, -1];
    let mut left = left as usize;
    let mut right = right as usize;
    while left < right {
        let mut i = left + 1;
        if primes[left] == false {
            left += 1;
        } else {
            while i < right && primes[i] == false {
                i += 1;
            }
            if primes[i] == true {
                if i - left < smallest_gap {
                    smallest_gap = i - left;
                    res[0] = left as i32;
                    res[1] = i as i32;
                }
            }
            left += 1;
        }
    }

    res
}
