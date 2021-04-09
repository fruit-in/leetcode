impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut max_len = 0;
        let mut ret = 0;

        for rec in rectangles {
            if rec[0].min(rec[1]) == max_len {
                ret += 1;
            } else if rec[0].min(rec[1]) > max_len {
                max_len = rec[0].min(rec[1]);
                ret = 1;
            }
        }

        ret
    }
}
