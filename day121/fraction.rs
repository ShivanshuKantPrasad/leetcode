// https://leetcode.com/problems/fraction-addition-and-subtraction/

use std::iter::Peekable;
use std::str::CharIndices;

struct Fraction {
    numerator: i32,
    denominator: i32,
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    (a * b) / gcd(a, b)
}
fn fraction(expression: &mut Peekable<CharIndices>) -> Fraction {
    let sign = sign(expression);
    let numerator = sign * number(expression);
    char(expression, '/');
    let denominator = number(expression);
    Fraction {
        numerator,
        denominator,
    }
}

fn sign(expression: &mut Peekable<CharIndices>) -> i32 {
    match expression.next_if(|(_, ch)| *ch == '+' || *ch == '-') {
        Some((_, x)) if x == '+' => 1,
        Some((_, x)) if x == '-' => -1,
        _ => 1,
    }
}

fn char(expression: &mut Peekable<CharIndices>, ch: char) -> Result<bool, String> {
    match expression.next() {
        Some((_, x)) if x == ch => Ok(true),
        Some((_, x)) => Err(format!("Unexpected Symbol '{x}' Expected {ch}")),
        None => Err("Unexpected end of expression.".to_string()),
    }
}

fn number(expression: &mut Peekable<CharIndices>) -> i32 {
    std::iter::from_fn(|| expression.by_ref().next_if(|(_, ch)| ch.is_numeric()))
        .map(|(_, c)| c)
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let mut iter = expression.char_indices().peekable();
        let mut result = fraction(&mut iter);

        while iter.peek().is_some() {
            let frac = fraction(&mut iter);
            let denominator = lcm(result.denominator, frac.denominator);
            let numerator = result.numerator * denominator / result.denominator
                + frac.numerator * denominator / frac.denominator;
            result.numerator = numerator;
            result.denominator = denominator;
        }

        let gcd = gcd(result.numerator, result.denominator).abs();
        result.numerator /= gcd;
        result.denominator /= gcd;

        format!("{}/{}", result.numerator, result.denominator)
    }
}
