impl Solution {
    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut prefix = vec![0_i32; s.len() + 1];
        let mut answer = vec![true; queries.len()];

        for (i, c) in s.bytes().enumerate() {
            prefix[i + 1] = prefix[i] ^ (1 << (c - b'a'));
        }

        for i in 0..queries.len() {
            let left = queries[i][0] as usize;
            let right = queries[i][1] as usize;
            let k = queries[i][2] as u32;

            answer[i] = (prefix[left] ^ prefix[right + 1]).count_ones() / 2 <= k;
        }

        answer
    }
}
