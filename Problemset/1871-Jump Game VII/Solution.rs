impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let mut indices = vec![0];

        for (i, _) in s.chars().enumerate().skip(1).filter(|&(_, c)| c == '0') {
            match indices.binary_search(&(i as i32 - max_jump)) {
                Err(j) if j == indices.len() || indices[j] > i as i32 - min_jump => (),
                _ => indices.push(i as i32),
            }
        }

        *indices.last().unwrap() == s.len() as i32 - 1
    }
}
