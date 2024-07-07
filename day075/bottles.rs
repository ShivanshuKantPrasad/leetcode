// https://leetcode.com/problems/water-bottles/description/

pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
    let mut drunk_bottles = 0;
    while num_bottles >= num_exchange {
        let exchanged = num_bottles / num_exchange;
        drunk_bottles += exchanged * num_exchange;
        num_bottles = num_bottles % num_exchange + exchanged;
    }
    drunk_bottles + num_bottles
}

fn main() {
    println!("{}", num_water_bottles(9, 3));
    println!("{}", num_water_bottles(15, 4));
}
