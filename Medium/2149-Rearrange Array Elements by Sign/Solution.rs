impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ps = vec![];
        let mut ns = vec![];
        let mut ret = vec![];

        for num in nums {
            if num > 0 {
                ps.push(num);
            } else {
                ns.push(num);
            }
        }

        for i in 0..ps.len() {
            ret.push(ps[i]);
            ret.push(ns[i]);
        }

        ret
    }
}
