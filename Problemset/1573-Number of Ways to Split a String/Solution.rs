impl Solution {
    pub fn num_ways(s: String) -> i32 {
        let mut indices = s
            .bytes()
            .enumerate()
            .filter(|&(i, c)| c == b'1')
            .map(|(i, c)| i as i32)
            .collect::<Vec<_>>();

        if indices.len() % 3 != 0 {
            return 0;
        }

        if indices.is_empty() {
            return ((s.len() - 2) as i64 * (s.len() - 1) as i64 / 2 % 1000000007) as i32;
        }

        let third = indices.len() / 3;
        let split_12 = (indices[third] - indices[third - 1]) as i64;
        let split_23 = (indices[2 * third] - indices[2 * third - 1]) as i64;

        (split_12 * split_23 % 1000000007) as i32
    }
}
