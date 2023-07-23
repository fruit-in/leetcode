impl Solution {
    pub fn rotate_string(a: String, b: String) -> bool {
        let mut c = a.clone();
        c.push_str(&a);
        c.len() == 2 * b.len() && c.contains(&b)
    }
}
