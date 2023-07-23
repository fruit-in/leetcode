impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut ret = 0;

        for x in gain {
            sum += x;
            ret = ret.max(sum);
        }

        ret
    }
}
