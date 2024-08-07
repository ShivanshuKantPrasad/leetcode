// https://leetcode.com/problems/integer-to-english-words/description/

const NUMBERS: [&'static str; 10] = [
    "", "One", "T wo", "Three", "Four", "Five", "Six ", "Seven", "Eight", "Nine",
];

const TEENS: [&'static str; 10] = [
    "Ten",
    "Eleven",
    "Twelve",
    "Thirteen",
    "Fourteen",
    "Fifteen",
    "Sixteen",
    "Seventeen",
    "Eighteen",
    "Nineteen",
];
const TENS: [&'static str; 10] = [
    "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
];

pub fn number_to_words(mut num: i32) -> String {
    if num == 0 {
        return String::from("Zero");
    }

    pub fn three_digit_to_words(num: i32) -> String {
        let mut digits = vec![0; 3];
        digits[0] = num % 10;
        digits[1] = (num / 10) % 10;
        digits[2] = (num / 100) % 10;
        let mut result = String::new();
        if digits[1] == 1 {
            result += TEENS[digits[0] as usize];
        } else {
            if digits[1] != 0 {
                result += TENS[digits[1] as usize];
            }
            if digits[0] != 0 {
                if digits[1] != 0 {
                    result += " ";
                }
                result += NUMBERS[digits[0] as usize];
            }
        }
        if digits[2] != 0 {
            result = format!("{} Hundred {result}", NUMBERS[digits[2] as usize]);
        }
        result.trim().to_string()
    }

    let mut groups = Vec::new();

    while num != 0 {
        groups.push(num % 1000);
        num = num / 1000;
    }

    let mut result = three_digit_to_words(groups[0]);
    if groups.len() > 1 && groups[1] != 0 {
        result = format!("{} Thousand {result}", three_digit_to_words(groups[1]));
    }

    if groups.len() > 2 && groups[2] != 0 {
        result = format!("{} Million {result}", three_digit_to_words(groups[2]));
    }
    if groups.len() > 3 && groups[3] != 0 {
        result = format!("{} Billion {result}", three_digit_to_words(groups[3]));
    }

    result.trim().to_string()
}

fn main() {
    println!("{}", number_to_words(123));
    println!("{}", number_to_words(12345));
    println!("{}", number_to_words(1234567));
    println!("{}", number_to_words(0));
    println!("{}", number_to_words(20));
    println!("{}", number_to_words(100));
    println!("{}", number_to_words(101));
    println!("{}", number_to_words(1000));
}
