impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let k = nums.len();
        let mut concat_nums = vec![];
        let mut subarray_counter = vec![0; k];
        let mut covered_subarrays = 0;
        let (mut a, mut b) = (-100000, 100000);

        for i in 0..k {
            for j in 0..nums[i].len() {
                concat_nums.push((nums[i][j], i));
            }
        }
        concat_nums.sort_unstable();

        let mut i = 0;

        for j in 0..concat_nums.len() {
            subarray_counter[concat_nums[j].1] += 1;
            if subarray_counter[concat_nums[j].1] == 1 {
                covered_subarrays += 1;
            }

            while covered_subarrays == k {
                let (c, d) = (concat_nums[i].0, concat_nums[j].0);
                if b - a > d - c || (b - a == d - c && a > c) {
                    (a, b) = (c, d);
                }

                subarray_counter[concat_nums[i].1] -= 1;
                if subarray_counter[concat_nums[i].1] == 0 {
                    covered_subarrays -= 1;
                }
                i += 1;
            }
        }

        vec![a, b]
    }
}
