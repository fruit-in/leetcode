impl Solution {
    pub fn circular_array_loop(mut nums: Vec<i32>) -> bool {
        let n = nums.len();

        for i in 0..nums.len() {
            nums[i] %= n as i32;
        }

        for i in 0..nums.len() {
            if nums[i] == 0 {
                continue;
            }

            let positive = nums[i] > 0;
            let mut j = i;

            while nums[j] != 0 && (nums[j] > 0) == positive {
                if nums[j].abs() > n as i32 {
                    return true;
                }

                if positive {
                    nums[j] += n as i32;
                    j = (j + nums[j] as usize) % n;
                } else {
                    nums[j] -= n as i32;
                    j = (j + n * 2 - nums[j].abs() as usize) % n;
                }
            }

            j = i;

            while nums[j] != 0 && (nums[j] > 0) == positive {
                let tmp = j;
                if positive {
                    j = (j + nums[j] as usize) % n;
                    nums[tmp] = 0;
                } else {
                    j = (j + n * 2 - nums[j].abs() as usize) % n;
                    nums[tmp] = 0;
                }
            }
        }

        false
    }
}
