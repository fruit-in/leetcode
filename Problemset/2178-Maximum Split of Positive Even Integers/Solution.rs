impl Solution {
    pub fn maximum_even_split(final_sum: i64) -> Vec<i64> {
        if final_sum % 2 == 1 {
            return vec![];
        }

        let mut final_sum = final_sum;
        let mut ret = vec![];

        for x in (2..=final_sum).step_by(2) {
            if final_sum - x <= x {
                ret.push(final_sum);
                break;
            }

            ret.push(x);
            final_sum -= x;
        }

        ret
    }
}
