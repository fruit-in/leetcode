impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let limit = limit as usize;
        let n = nums.len();
        let mut arr = vec![0; limit * 2 + 2];
        let mut prefix_sum = 0;
        let mut ret = i32::MAX;

        for i in 0..n / 2 {
            let x = nums[i] as usize;
            let y = nums[n - 1 - i] as usize;

            arr[2] += 2;
            arr[x.min(y) + 1] -= 1;
            arr[x.max(y) + limit + 1] += 1;
            arr[x + y] -= 1;
            arr[x + y + 1] += 1;
        }

        for i in 2..=limit * 2 {
            prefix_sum += arr[i];
            ret = ret.min(prefix_sum);
        }

        ret
    }
}
