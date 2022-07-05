impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut ret = vec![];
        digits.sort_unstable();

        for i in 0..digits.len() {
            for j in 0..digits.len() {
                for k in 0..digits.len() {
                    let x = digits[i] * 100 + digits[j] * 10 + digits[k];

                    if i != j && j != k && i != k && x % 2 == 0 && x > *ret.last().unwrap_or(&99) {
                        ret.push(x);
                    }
                }
            }
        }

        ret
    }
}
