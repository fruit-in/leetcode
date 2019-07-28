impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut counter = 0;
        let mut result = String::new();
        for ch in s.chars() {
            if ch == ')' {
                counter += 1;
            }
            if counter != 0 {
                result.push(ch);
            }
            if ch == '(' {
                counter -= 1;
            }
        }
        result
    }
}
