impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut longest = 0;
        let mut indexes = vec![0; 32];
        let mut i = 0;
        for j in 0..32 {
            if (n >> j) & 1 == 1 {
                indexes[i] = j;
                i += 1;
            }
        }
        i = 1;
        while indexes[i] > indexes[i - 1] {
            longest = longest.max(indexes[i] - indexes[i - 1]);
            i += 1;
        }
        longest
    }
}
