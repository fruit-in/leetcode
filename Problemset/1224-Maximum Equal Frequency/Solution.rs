use std::collections::HashMap;

impl Solution {
    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let mut count_nums = HashMap::new();
        let mut count_ocurrences = HashMap::new();

        for &num in &nums {
            *count_nums.entry(num).or_insert(0) += 1;
        }
        for &ocurrence in count_nums.values() {
            *count_ocurrences.entry(ocurrence).or_insert(0) += 1;
        }

        for i in (0..nums.len()).rev() {
            if count_ocurrences.len() == 1 {
                let (&k0, &v0) = count_ocurrences.iter().next().unwrap();

                if k0 == 1 || v0 == 1 {
                    return i as i32 + 1;
                }
            } else if count_ocurrences.len() == 2 {
                let (&k0, &v0) = count_ocurrences.iter().max().unwrap();
                let (&k1, &v1) = count_ocurrences.iter().min().unwrap();

                if (k1, v1) == (1, 1) || (k0 == k1 + 1 && v0 == 1) {
                    return i as i32 + 1;
                }
            }

            *count_ocurrences.get_mut(&count_nums[&nums[i]]).unwrap() -= 1;
            if count_ocurrences.get(&(count_nums[&nums[i]])) == Some(&0) {
                count_ocurrences.remove(&count_nums[&nums[i]]);
            }
            *count_nums.get_mut(&nums[i]).unwrap() -= 1;
            if count_nums[&nums[i]] > 0 {
                *count_ocurrences.entry(count_nums[&nums[i]]).or_insert(0) += 1;
            }
        }

        1
    }
}
