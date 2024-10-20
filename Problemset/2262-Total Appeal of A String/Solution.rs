impl Solution {
    pub fn appeal_sum(s: String) -> i64 {
        let s = s.as_bytes();
        let mut last = vec![-1; 27];
        let mut mask = 0_i32;
        let mut ret = 0;

        for i in 0..s.len() {
            last[(s[i] - b'a') as usize] = i as i64;
            mask |= 1 << (s[i] - b'a');
            let mut tmp = last.clone();
            let mut appeal = mask.count_ones() as i64;
            tmp.sort_unstable();

            for j in 1..27 {
                if tmp[j] > -1 {
                    ret += (tmp[j] - tmp[j - 1]) * appeal;
                    appeal -= 1;
                }
            }
        }

        ret
    }
}
