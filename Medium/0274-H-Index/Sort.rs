impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations = citations;
        citations.sort_unstable_by(|a, b| b.cmp(a));
        let mut h = 0;

        for i in 0..citations.len() {
            h = h.max(citations[i].min(i as i32 + 1));
        }

        h
    }
}
