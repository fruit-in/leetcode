# 414. 第三大的数
给定一个非空数组，返回此数组中第三大的数。如果不存在，则返回数组中最大的数。要求算法时间复杂度必须是O(n)。

#### 示例 1:
<pre>
<strong>输入:</strong> [3, 2, 1]
<strong>输出:</strong> 1
<strong>解释:</strong> 第三大的数是 1.
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [1, 2]
<strong>输出:</strong> 2
<strong>解释:</strong> 第三大的数不存在, 所以返回最大的数 2.
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> [2, 2, 3, 1]
<strong>输出:</strong> 1
<strong>解释:</strong> 注意，要求返回第三大的数，是指第三大且唯一出现的数。
存在两个值为2的数，它们都排第二。
</pre>

## 题解 (Rust)

### 1. 题解
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
