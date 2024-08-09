impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let mut i = k;
        let mut j = k;
        let mut min_num = nums[k as usize];
        let mut ret = min_num;

        loop {
            while i - 1 >= 0 && nums[i as usize - 1] >= min_num {
                i -= 1;
            }
            while j + 1 < nums.len() as i32 && nums[j as usize + 1] >= min_num {
                j += 1;
            }

            ret = ret.max(min_num * (j - i + 1));
            i -= 1;
            if i < 0 {
                break;
            }
            min_num = nums[i as usize];
        }

        i = k;
        j = k;
        min_num = nums[k as usize];

        loop {
            while i - 1 >= 0 && nums[i as usize - 1] >= min_num {
                i -= 1;
            }
            while j + 1 < nums.len() as i32 && nums[j as usize + 1] >= min_num {
                j += 1;
            }

            ret = ret.max(min_num * (j - i + 1));
            j += 1;
            if j >= nums.len() as i32 {
                break;
            }
            min_num = nums[j as usize];
        }

        ret
    }
}
