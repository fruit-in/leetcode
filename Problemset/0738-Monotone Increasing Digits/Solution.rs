impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut digits = n.to_string().into_bytes();
        let mut i = digits.len();

        while let Some(j) = (0..digits.len() - 1).find(|&j| digits[j] > digits[j + 1]) {
            digits[j] -= 1;
            for k in j + 1..i {
                digits[k] = b'9';
            }
            i = j + 1;
        }

        String::from_utf8(digits).unwrap().parse().unwrap()
    }
}
