impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let word = word.as_bytes();
        let mut count = [0; 1024];
        let mut mask = 0;
        let mut ret = 0;
        count[0] = 1;

        for i in 0..word.len() {
            mask ^= 1 << (word[i] - b'a');
            for j in 0..10 {
                ret += count[mask ^ (1 << j)];
            }
            ret += count[mask];
            count[mask] += 1;
        }

        ret
    }
}
