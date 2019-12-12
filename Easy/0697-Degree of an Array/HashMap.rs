use std::collections::HashMap;

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut cnt_left_len = HashMap::new();
        let mut degree = 0;

        for i in 0..nums.len() {
            let cll = cnt_left_len.entry(nums[i]).or_insert([0, i, 1]);
            cll[0] += 1;
            cll[2] = i - cll[1] + 1;
            degree = degree.max(cll[0])
        }

        cnt_left_len.values()
                    .filter(|arr| arr[0] == degree)
                    .map(|arr| arr[2])
                    .min()
                    .unwrap() as i32
    }
}
