impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut s = s;
        while s.contains("()") {
            s = s.replace("()", "");
        }
        s.len() as i32
    }
}
