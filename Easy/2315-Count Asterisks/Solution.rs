impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut in_pair = false;
        let mut ret = 0;

        for c in s.chars() {
            if !in_pair && c == '*' {
                ret += 1;
            } else if c == '|' {
                in_pair = !in_pair;
            }
        }

        ret
    }
}
