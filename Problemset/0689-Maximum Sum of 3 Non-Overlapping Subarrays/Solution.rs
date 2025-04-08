impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut subarray_sum = vec![0; nums.len() + 1 - k];
        let mut suffix_max1 = vec![(0, 0); nums.len() + 2 - k];
        let mut suffix_max2 = vec![(0, 0, 0); nums.len() + 2 - k * 2];
        let mut suffix_max3 = vec![(0, 0, 0, 0); nums.len() + 2 - k * 3];

        subarray_sum[0] = (0..k).map(|i| nums[i]).sum();
        for i in 1..subarray_sum.len() {
            subarray_sum[i] = subarray_sum[i - 1] - nums[i - 1] + nums[i + k - 1];
        }

        for i in (0..suffix_max1.len() - 1).rev() {
            suffix_max1[i] = suffix_max1[i + 1];
            if subarray_sum[i] >= suffix_max1[i].0 {
                suffix_max1[i] = (subarray_sum[i], i);
            }

            if i < suffix_max2.len() - 1 {
                suffix_max2[i] = suffix_max2[i + 1];
                if subarray_sum[i] + suffix_max1[i + k].0 >= suffix_max2[i].0 {
                    suffix_max2[i] = (
                        subarray_sum[i] + suffix_max1[i + k].0,
                        i,
                        suffix_max1[i + k].1,
                    );
                }
            }

            if i < suffix_max3.len() - 1 {
                suffix_max3[i] = suffix_max3[i + 1];
                if subarray_sum[i] + suffix_max2[i + k].0 >= suffix_max3[i].0 {
                    suffix_max3[i] = (
                        subarray_sum[i] + suffix_max2[i + k].0,
                        i,
                        suffix_max2[i + k].1,
                        suffix_max2[i + k].2,
                    );
                }
            }
        }

        vec![
            suffix_max3[0].1 as i32,
            suffix_max3[0].2 as i32,
            suffix_max3[0].3 as i32,
        ]
    }
}
