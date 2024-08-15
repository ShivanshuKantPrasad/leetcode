// https://leetcode.com/problems/lemonade-change/

pub fn lemonade_change(bills: Vec<i32>) -> bool {
    let mut register = vec![0; 3];

    for bill in bills {
        if bill == 5 {
            register[0] += 1;
            continue;
        }
        if bill == 10 {
            if register[0] > 0 {
                register[0] -= 1;
                register[1] += 1;
            } else {
                return false;
            }
        }
        if bill == 20 {
            if register[1] > 0 && register[0] > 0 {
                register[0] -= 1;
                register[1] -= 1;
                register[2] += 1;
            } else if register[0] >= 3 {
                register[0] -= 3;
                register[2] += 1;
            } else {
                return false;
            }
        }
    }

    true
}
