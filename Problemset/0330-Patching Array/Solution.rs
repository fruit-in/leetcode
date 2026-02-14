impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let nums = nums.into_iter().map(|num| num as i64).collect::<Vec<_>>();
        let n = n as i64;
        let mut sum = 0;
        let mut ret = 0;

        for &num in &nums {
            while sum < num - 1 && sum < n {
                sum += sum + 1;
                ret += 1;
            }
            sum += num;

            if sum >= n {
                break;
            }
        }

        while sum < n {
            sum += sum + 1;
            ret += 1;
        }

        ret
    }
}
