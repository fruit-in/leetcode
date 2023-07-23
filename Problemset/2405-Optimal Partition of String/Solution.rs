impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut mask = 0;
        let mut ret = 1;

        for c in s.bytes() {
            if (1 << (c - b'a')) & mask != 0 {
                mask = 0;
                ret += 1;
            }

            mask |= 1 << (c - b'a');
        }

        ret
    }
}
