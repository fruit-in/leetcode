impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let rule_key = match rule_key.as_str() {
            "type" => 0,
            "color" => 1,
            _ => 2,
        };

        items
            .iter()
            .filter(|item| item[rule_key] == rule_value)
            .count() as i32
    }
}
