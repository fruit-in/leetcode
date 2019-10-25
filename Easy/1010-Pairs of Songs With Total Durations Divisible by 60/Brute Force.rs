impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut ret = 0;

        for i in 0..time.len() {
            for j in (i + 1)..time.len() {
                if (time[i] + time[j]) % 60 == 0 {
                    ret += 1;
                }
            }
        }

        ret
    }
}
