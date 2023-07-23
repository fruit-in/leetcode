impl Solution {
    pub fn smallest_number(num: i64) -> i64 {
        let mut digits = num.abs().to_string().into_bytes();

        digits.sort_unstable();
        if num >= 0 {
            for i in 0..digits.len() {
                if digits[i] != b'0' {
                    digits.swap(0, i);
                    break;
                }
            }
        } else {
            digits.push(b'-');
            digits.reverse();
        }

        String::from_utf8(digits).unwrap().parse().unwrap()
    }
}
