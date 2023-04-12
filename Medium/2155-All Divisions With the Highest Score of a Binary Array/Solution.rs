impl Solution {
    pub fn max_score_indices(nums: Vec<i32>) -> Vec<i32> {
        let mut left0 = 0;
        let mut right1 = nums.iter().filter(|&&x| x == 1).count();
        let mut max_score = right1;
        let mut ret = vec![0];

        for i in 0..nums.len() {
            match nums[i] {
                0 => left0 += 1,
                _ => right1 -= 1,
            }

            if left0 + right1 > max_score {
                max_score = left0 + right1;
                ret = vec![i as i32 + 1];
            } else if left0 + right1 == max_score {
                ret.push(i as i32 + 1);
            }
        }

        ret
    }
}
