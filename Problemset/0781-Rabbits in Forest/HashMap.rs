use std::collections::HashMap;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut same_tell = HashMap::new();

        for answer in answers {
            *same_tell.entry(answer + 1).or_insert(0) += 1;
        }

        same_tell
            .iter()
            .map(|(&k, &v)| (v as f64 / k as f64).ceil() as i32 * k)
            .sum()
    }
}
