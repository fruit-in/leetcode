use std::collections::HashSet;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut possible_counts = HashSet::from([0]);

        for c in s.chars() {
            let mut next_counts = HashSet::new();

            for count in possible_counts.into_iter() {
                if c == '(' {
                    next_counts.insert(count + 1);
                } else if c == ')' && count > 0 {
                    next_counts.insert(count - 1);
                } else if c == '*' {
                    next_counts.insert(count);
                    next_counts.insert(count + 1);
                    next_counts.insert((count - 1).max(0));
                }
            }

            possible_counts = next_counts;
        }

        possible_counts.contains(&0)
    }
}
