use std::collections::VecDeque;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let n = senate.len() as usize;
        let mut radiant = VecDeque::new();
        let mut dire = VecDeque::new();

        for (i, senator) in senate.chars().enumerate() {
            if senator == 'R' {
                radiant.push_back(i);
            } else {
                dire.push_back(i);
            }
        }

        while !radiant.is_empty() && !dire.is_empty() {
            if radiant[0] < dire[0] {
                radiant.push_back(radiant[0] + n);
            } else {
                dire.push_back(dire[0] + n);
            }
            radiant.pop_front();
            dire.pop_front();
        }

        if dire.is_empty() {
            "Radiant".to_string()
        } else {
            "Dire".to_string()
        }
    }
}
