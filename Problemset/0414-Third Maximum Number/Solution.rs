impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut max_nums = Vec::new();
        for n in nums {
            if !max_nums.contains(&n) {
                match max_nums.len() {
                    0 => max_nums.push(n),
                    1 => {
                        if max_nums[0] < n {
                            max_nums.push(n);
                        } else {
                            max_nums.insert(0, n);
                        }
                    },
                    2 => {
                        if max_nums[1] < n {
                            max_nums.push(n);
                        } else if max_nums[0] > n {
                            max_nums.insert(0, n);
                        } else {
                            max_nums.insert(1, n);
                        }
                    },
                    3 => {
                        if max_nums[2] < n {
                            max_nums.push(n);
                            max_nums.remove(0);
                        } else if max_nums[1] < n {
                            max_nums.insert(2, n);
                            max_nums.remove(0);
                        } else if max_nums[0] < n {
                            max_nums.insert(1, n);
                            max_nums.remove(0);
                        }
                    },
                    _ => (),
                };
            }
        }
        if max_nums.len() < 3 {
            *max_nums.last().unwrap()
        } else {
            *max_nums.first().unwrap()
        }
    }
}
