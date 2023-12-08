impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        for x in 1..=n {
            if !s.contains(&format!("{:b}", x)) {
                return false;
            }
        }

        true
    }
}
