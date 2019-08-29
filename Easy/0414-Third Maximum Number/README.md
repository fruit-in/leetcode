# 414. Third Maximum Number
Given a **non-empty** array of integers, return the **third** maximum number in this array. If it does not exist, return the maximum number. The time complexity must be in O(n).

#### Example 1:
<pre>
<strong>Input:</strong> [3, 2, 1]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The third maximum is 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [1, 2]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The third maximum does not exist, so the maximum (2) is returned instead.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> [2, 2, 3, 1]
<strong>Output:</strong> 1
<strong>Explanation:</strong> Note that the third maximum here means the third maximum distinct number.
Both numbers with value 2 are both considered as second maximum.
</pre>

## Solutions (Rust)

### 1. Solution
```Rust
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
```
