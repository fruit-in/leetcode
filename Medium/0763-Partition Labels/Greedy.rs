impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last = [0; 26];

        for (i, ch) in s.bytes().enumerate() {
            last[(ch - b'a') as usize] = i;
        }

        let mut l = 0;
        let mut r = 0;
        let mut ret = Vec::new();

        for (i, ch) in s.bytes().enumerate() {
            if i > r {
                ret.push((r - l) as i32 + 1);
                l = i;
                r = last[(ch - b'a') as usize];
            } else if last[(ch - b'a') as usize] > r {
                r = last[(ch - b'a') as usize];
            }
        }

        ret.push((r - l) as i32 + 1);

        ret
    }
}
