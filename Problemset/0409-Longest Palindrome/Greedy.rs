use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut count = HashMap::new();
        let mut result = 0;
        for ch in s.chars() {
            match count.get(&ch) {
                Some(i) => count.insert(ch, i + 1),
                None => count.insert(ch, 1),
            };
        }
        for v in count.values() {
            if v % 2 == 0 || result % 2 == 0 {
                result += v;
            } else {
                result += v - 1;
            }
        }
        result
    }
}
