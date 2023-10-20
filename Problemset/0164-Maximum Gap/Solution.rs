impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let mut nums0 = nums.into_iter().map(|x| x as usize).collect::<Vec<_>>();
        let mut x = 0xff;

        for i in (0..32).step_by(8) {
            let mut count = vec![0; 256];
            let mut nums1 = vec![0; nums0.len()];

            for j in 0..nums0.len() {
                count[(nums0[j] & x) >> i] += 1;
            }

            for j in 1..count.len() {
                count[j] += count[j - 1];
            }

            for j in (0..nums0.len()).rev() {
                count[(nums0[j] & x) >> i] -= 1;
                nums1[count[(nums0[j] & x) >> i]] = nums0[j];
            }

            nums0 = nums1;
            x <<= 8;
        }

        (1..nums0.len())
            .map(|i| nums0[i] - nums0[i - 1])
            .max()
            .unwrap_or(0) as i32
    }
}
