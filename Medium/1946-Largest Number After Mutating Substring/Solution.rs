impl Solution {
    pub fn maximum_number(num: String, change: Vec<i32>) -> String {
        let mut digits = num.into_bytes();

        for i in 0..digits.len() {
            if digits[i] < change[(digits[i] - b'0') as usize] as u8 + b'0' {
                for j in i..digits.len() {
                    if digits[j] > change[(digits[j] - b'0') as usize] as u8 + b'0' {
                        break;
                    }

                    digits[j] = change[(digits[j] - b'0') as usize] as u8 + b'0';
                }

                break;
            }
        }

        String::from_utf8(digits).unwrap()
    }
}
