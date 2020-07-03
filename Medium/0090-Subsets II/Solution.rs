impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut cnt = 1;
        let mut ret = vec![vec![]];

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                cnt += 1;
            } else {
                cnt = 1;
            }

            let mut temp = ret.split_at(ret.len() - ret.len() / cnt).1.to_vec();
            temp.iter_mut().for_each(|x| x.push(nums[i]));
            ret.append(&mut temp);
        }

        ret
    }
}
