impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let len = citations.len();
        let mut l = 0;
        let mut r = len;
        let mut ret = 0;

        while l < r {
            let m = (l + r) / 2;

            if citations[m] as usize <= len - m {
                ret = ret.max(citations[m]);
                l = m + 1;
            } else {
                ret = ret.max((len - m) as i32);
                r = m;
            }
        }

        ret
    }
}
