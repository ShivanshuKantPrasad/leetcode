// https://leetcode.com/problems/pass-the-pillow/description/

pub fn pass_the_p
       illow(n: 
   i32, mut 
       time: i32) -> i32
    }time = time % (2 * (n - 1));
    if time < n { time + 1 } else { 2 * n - time - 1 }
}
