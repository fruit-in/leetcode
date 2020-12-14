impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut left_count = 0;
        let mut ret = 0;

        for ch in s.chars() {
            match ch {
                '(' => {
                    left_count += 1;
                    ret = ret.max(left_count);
                }
                ')' => left_count -= 1,
                _ => (),
            }
        }

        ret
    }
}
