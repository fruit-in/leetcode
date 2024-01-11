impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut max_len = vec![1; nums.len()];
        let mut prev_index = vec![None; nums.len()];
        let mut answer = vec![];

        nums.sort_unstable();

        for i in 1..nums.len() {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && max_len[i] < max_len[j] + 1 {
                    max_len[i] = max_len[j] + 1;
                    prev_index[i] = Some(j);
                }
            }
        }

        let mut curr = (0..nums.len()).max_by_key(|&i| max_len[i]).unwrap();
        answer.push(nums[curr]);

        while let Some(i) = prev_index[curr] {
            curr = i;
            answer.push(nums[curr]);
        }

        answer
    }
}
