use std::collections::HashMap;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let negative = numerator != 0 && ((numerator < 0) ^ (denominator < 0));
        let mut numerator = (numerator as i64).abs();
        let denominator = (denominator as i64).abs();
        let mut numerators = HashMap::new();
        let mut fraction = vec![];

        if negative {
            fraction.push(b'-');
        }
        fraction.append(&mut (numerator / denominator).to_string().into_bytes());
        if numerator % denominator != 0 {
            fraction.push(b'.');
        }
        numerator = (numerator % denominator) * 10;

        while numerator > 0 {
            if let Some(&i) = numerators.get(&numerator) {
                fraction.insert(i, b'(');
                fraction.push(b')');
                break;
            } else {
                fraction.push((numerator / denominator) as u8 + b'0');
                numerators.insert(numerator, fraction.len() - 1);
                numerator = (numerator % denominator) * 10;
            }
        }

        String::from_utf8(fraction).unwrap()
    }
}
