impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut left = 0;
        let mut right = 0;
        for ch in s.chars() {
            if ch == '(' {
                left += 1
            } else if left > 0{
                left -= 1
            } else {
                right += 1
            }
        }
        left + right
    }
}
