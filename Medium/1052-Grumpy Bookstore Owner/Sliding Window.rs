impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
        let mut curr = 0;
        let mut ret = 0;

        for i in 0..grumpy.len() {
            if i < x as usize || grumpy[i] == 0 {
                curr += customers[i];
                ret += customers[i];
            }
            if i >= x as usize && grumpy[i - x as usize] == 1 {
                curr -= customers[i - x as usize];
            }
            if i >= x as usize && grumpy[i] == 1 {
                curr += customers[i];
            }

            ret = ret.max(curr);
        }

        ret
    }
}
