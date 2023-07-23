impl Solution {
    pub fn to_lower_case(str: String) -> String {
        let mut lower_case_str = String::new();
        for ch in str.chars() {
            lower_case_str.push(ch.to_ascii_lowercase());
        }
        lower_case_str
    }
}
