impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut stack: Vec<char> = Vec::new();
        for ch in s.chars() {
            if ch == ')' && stack.ends_with(&['(']) {
                stack.pop();
            } else {
                stack.push(ch);
            }
        }
        stack.len() as i32
    }
}
