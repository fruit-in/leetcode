impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut i = digits.len() - 1;
        loop {
            if digits[i] != 9 {
                digits[i] += 1;
                return digits;
            } else {
                digits[i] = 0;
                if i > 0 {
                    i -= 1;
                } else {
                    digits.insert(0, 1);
                    return digits;
                }
            }
        }
    }
}
