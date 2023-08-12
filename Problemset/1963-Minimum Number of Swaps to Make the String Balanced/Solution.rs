impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut count = 0;
        let mut ret = 0;

        for c in s.chars() {
            if c == '[' {
                count += 1;
            } else if c == ']' && count > 0 {
                count -= 1;
            } else {
                count += 1;
                ret += 1;
            }
        }

        ret
    }
}
