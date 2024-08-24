// https://leetcode.com/problems/find-the-closest-palindrome/description/

pub fn nearest_palindromic(n: String) -> String {
    fn convert(num: i64) -> i64 {
        let s = num.to_string();
        let n = s.len() as i32;
        let mut l = (n - 1) / 2;
        let mut r = n / 2;
        let mut s_list = s.into_bytes();
        println!("{l} {r}");
        while l >= 0 {
            s_list[r as usize] = s_list[l as usize];
            r += 1;
            l -= 1;
        }
        String::from_utf8(s_list).unwrap().parse::<i64>().unwrap()
    }

    fn next_palindrome(num: i64) -> i64 {
        let mut left = 0;
        let mut right = num;
        let mut ans = i64::MIN;
        while left <= right {
            let mid = (right - left) / 2 + left;
            let palin = convert(mid);
            if palin < num {
                ans = palin;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        ans
    }

    fn min_binary_search(num: i64) -> i64 {
        let mut left = num;
        let mut right = 1000_000_000_000_000_000;
        let mut ans = i64::MIN;
        while left <= right {
            let mid = (right - left) / 2 + left;
            let palin = convert(mid);
            if palin > num {
                ans = palin;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        ans
    }

    let num = n.parse::<i64>().unwrap();
    let a = next_palindrome(num);
    let b = min_binary_search(num);
    if i64::abs(a - num) <= i64::abs(b - num) {
        a.to_string()
    } else {
        b.to_string()
    }
}
