impl Solution {
    pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        let mut max_score = 0;
        let mut ret = i32::MAX;

        for i in 0..divisors.len() {
            let mut score = 0;

            for j in 0..nums.len() {
                if nums[j] % divisors[i] == 0 {
                    score += 1;
                }
            }

            if score > max_score {
                max_score = score;
                ret = divisors[i];
            } else if score == max_score && divisors[i] < ret {
                ret = divisors[i];
            }
        }

        ret
    }
}
