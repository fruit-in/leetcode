impl Solution {
    pub fn num_splits(s: String) -> i32 {
        let mut p_count = vec![];
        let mut q_count = vec![];
        let mut count = 0i32;

        for ch in s.bytes().take(s.len() - 1) {
            count |= 1 << (ch - b'a');
            p_count.push(count.count_ones());
        }
        count = 0;
        for ch in s.bytes().rev().take(s.len() - 1) {
            count |= 1 << (ch - b'a');
            q_count.push(count.count_ones());
        }

        p_count
            .iter()
            .zip(q_count.iter().rev())
            .filter(|(x, y)| x == y)
            .count() as i32
    }
}
