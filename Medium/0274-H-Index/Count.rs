impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut counter = vec![0; citations.len() + 1];
        for &c in &citations {
            counter[(c as usize).min(citations.len())] += 1;
        }

        let mut s = 0;
        let mut i = counter.len() - 1;
        loop {
            s += counter[i];
            if s >= i {
                return i as i32;
            }
            i -= 1;
        }
    }
}
