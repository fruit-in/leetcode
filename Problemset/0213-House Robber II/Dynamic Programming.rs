impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut rob_fst = (0, 0, nums[0]);
        let mut rob_lst = (0, 0, 0);

        for i in 1..nums.len() {
            if i != nums.len() - 1 {
                rob_fst = (rob_fst.1, rob_fst.2, nums[i] + rob_fst.0.max(rob_fst.1));
            }
            rob_lst = (rob_lst.1, rob_lst.2, nums[i] + rob_lst.0.max(rob_lst.1));
        }

        rob_fst.1.max(rob_fst.2).max(rob_lst.1).max(rob_lst.2)
    }
}
