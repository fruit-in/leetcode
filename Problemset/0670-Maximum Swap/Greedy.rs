impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut digits = num.to_string().into_bytes();
        let mut last = [0; 10];

        for i in 0..digits.len() {
            last[(digits[i] - b'0') as usize] = i;
        }

        for i in 0..digits.len() {
            for j in (((digits[i] - b'0') as usize + 1)..=9).rev() {
                if last[j] > i {
                    digits.swap(i, last[j]);
                    return String::from_utf8(digits).unwrap().parse().unwrap();
                }
            }
        }

        num
    }
}
