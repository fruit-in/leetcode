# 2342. 数位和相等数对的最大和
给你一个下标从 **0** 开始的数组 `nums` ，数组中的元素都是 **正** 整数。请你选出两个下标 `i` 和 `j`（`i != j`），且 `nums[i]` 的数位和 与  `nums[j]` 的数位和相等。

请你找出所有满足条件的下标 `i` 和 `j` ，找出并返回 `nums[i] + nums[j]` 可以得到的 **最大值** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [18,43,36,13,7]
<strong>输出:</strong> 54
<strong>解释:</strong> 满足条件的数对 (i, j) 为：
- (0, 2) ，两个数字的数位和都是 9 ，相加得到 18 + 36 = 54 。
- (1, 4) ，两个数字的数位和都是 7 ，相加得到 43 + 7 = 50 。
所以可以获得的最大和是 54 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [10,12,19,14]
<strong>输出:</strong> -1
<strong>解释:</strong> 不存在满足条件的数对，返回 -1 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut max_pairs = HashMap::new();

        for &num in &nums {
            let digits_sum = num
                .to_string()
                .bytes()
                .map(|d| (d - b'0') as i32)
                .sum::<i32>();
            let pair = max_pairs.entry(digits_sum).or_insert((0, 0));

            if num >= pair.1 {
                *pair = (pair.1, num);
            } else if num > pair.0 {
                *pair = (num, pair.1);
            }
        }

        max_pairs
            .values()
            .filter(|&&(x, y)| x > 0)
            .map(|(x, y)| x + y)
            .max()
            .unwrap_or(-1)
    }
}
```
