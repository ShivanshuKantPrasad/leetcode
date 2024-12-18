// https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/description/

pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
    for i in 0..prices.len() {
        for j in (i + 1)..prices.len() {
            if prices[j] <= prices[i] {
                prices[i] -= prices[j];
                break;
            }
        }
    }
    prices
}
