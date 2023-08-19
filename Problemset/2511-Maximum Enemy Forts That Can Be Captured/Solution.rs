impl Solution {
    pub fn capture_forts(forts: Vec<i32>) -> i32 {
        let mut prev = (0, 0);
        let mut ret = 0;

        for i in 0..forts.len() {
            if forts[i] != 0 {
                if prev.0 != 0 && prev.0 != forts[i] {
                    ret = ret.max(i - prev.1 - 1);
                }

                prev = (forts[i], i);
            }
        }

        ret as i32
    }
}
