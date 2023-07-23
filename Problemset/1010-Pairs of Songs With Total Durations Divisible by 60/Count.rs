impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut cnt = [0; 60];
        let mut ret = 0;

        for t in time {
            ret += match t % 60 {
                0 => cnt[0],
                _ => cnt[60 - t as usize % 60],
            };
            cnt[t as usize % 60] += 1;
        }

        ret
    }
}
