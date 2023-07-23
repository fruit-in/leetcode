impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut count = [(0_i32, 0_i32); 26];

        s.bytes().for_each(|b| count[(b - b'a') as usize].0 += 1);
        t.bytes().for_each(|b| count[(b - b'a') as usize].1 += 1);

        count.into_iter().map(|(x, y)| (x - y).abs()).sum()
    }
}
