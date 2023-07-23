impl Solution {
    pub fn num_times_all_blue(light: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut ret = 0;

        for k in 0..light.len() {
            max = max.max(light[k]);
            if max == k as i32 + 1 {
                ret += 1;
            }
        }

        ret
    }
}
