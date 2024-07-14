// https://leetcode.com/problems/number-of-atoms/

pub fn count_of_atoms(formula: String) -> String {
    use std::collections::HashMap;

    fn count_of_atoms(formula: &[u8]) -> HashMap<String, i32> {
        let mut freq = HashMap::new();
        let mut i = 0;
        while i < formula.len() {
            // println!("{:?} {i}", String::from_utf8(Vec::from(formula)));
            if formula[i] == b'(' {
                i += 1;
                let start = i;
                let mut stack = 1;
                while i < formula.len() {
                    match formula[i] {
                        b'(' => stack += 1,
                        b')' => {
                            stack -= 1;
                            if stack == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }
                let end = i;
                i += 1;
                let req = count_of_atoms(&formula[start..end]);
                let mut mult = 0;
                while i < formula.len() && (formula[i] as char).is_numeric() {
                    mult = mult * 10 + (formula[i] - b'0') as i32;
                    i += 1;
                }
                mult = mult.max(1);
                for (key, val) in req.iter() {
                    *freq.entry(key.clone()).or_insert(0) += val * mult;
                }
            } else if (formula[i] as char).is_uppercase() {
                let start = i;
                i += 1;
                if i < formula.len() && (formula[i] as char).is_lowercase() {
                    i += 1;
                }
                let end = i;
                let mut mult = 0;
                while i < formula.len() && (formula[i] as char).is_numeric() {
                    mult = mult * 10 + (formula[i] - b'0') as i32;
                    i += 1;
                }
                mult = mult.max(1);
                let key = formula[start..end].to_vec();
                let key = String::from_utf8(key).unwrap();
                *freq.entry(key).or_insert(0) += mult;
            }
        }
        freq
    }

    let freq = count_of_atoms(formula.as_bytes());
    // println!("{freq:?}");
    let mut freq = freq.iter().collect::<Vec<_>>();
    // println!("{freq:?}");
    freq.sort_unstable_by_key(|x| x.0);
    freq.iter()
        .map(|x| {
            if *x.1 > 1 {
                format!("{}{}", x.0, x.1)
            } else {
                format!("{}", x.0)
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

fn main() {
    println!("{}", count_of_atoms(String::from("H2O")));
    println!("{}", count_of_atoms(String::from("Mg(OH)2")));
    println!("{}", count_of_atoms(String::from("K4(ON(SO3)2)2")));
    println!("{}", count_of_atoms(String::from("((N42)24(OB40Li30CHe3O48LiNN26)33(C12Li48N30H13HBe31)21(BHN30Li26BCBe47N40)15(H5)16)14")));
}
