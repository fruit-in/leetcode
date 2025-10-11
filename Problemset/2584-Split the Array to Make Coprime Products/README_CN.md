# 2584. 分割数组使乘积互质
给你一个长度为 `n` 的整数数组 `nums` ，下标从 **0** 开始。

如果在下标 `i` 处 **分割** 数组，其中 `0 <= i <= n - 2` ，使前 `i + 1` 个元素的乘积和剩余元素的乘积互质，则认为该分割 **有效** 。

* 例如，如果 `nums = [2, 3, 3]` ，那么在下标 `i = 0` 处的分割有效，因为 `2` 和 `9` 互质，而在下标 `i = 1` 处的分割无效，因为 `6` 和 `3` 不互质。在下标 `i = 2` 处的分割也无效，因为 `i == n - 1` 。

返回可以有效分割数组的最小下标 `i` ，如果不存在有效分割，则返回 `-1` 。

当且仅当 `gcd(val1, val2) == 1` 成立时，`val1` 和 `val2` 这两个值才是互质的，其中 `gcd(val1, val2)` 表示 `val1` 和 `val2` 的最大公约数。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/12/14/second.PNG)
<pre>
<strong>输入:</strong> nums = [4,7,8,15,3,5]
<strong>输出:</strong> 2
<strong>解释:</strong> 上表展示了每个下标 i 处的前 i + 1 个元素的乘积、剩余元素的乘积和它们的最大公约数的值。
唯一一个有效分割位于下标 2 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/12/14/capture.PNG)
<pre>
<strong>输入:</strong> nums = [4,7,15,8,3,5]
<strong>输出:</strong> -1
<strong>解释:</strong> 上表展示了每个下标 i 处的前 i + 1 个元素的乘积、剩余元素的乘积和它们的最大公约数的值。
不存在有效分割。
</pre>

#### 提示:
* `n == nums.length`
* <code>1 <= n <= 10<sup>4</sup></code>
* <code>1 <= nums[i] <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_valid_split(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut range = HashMap::new();
        let mut arr = vec![0; n + 1];
        let mut prefix_sum = 0;

        for i in 0..n {
            for x in 1..=nums[i].isqrt() {
                if nums[i] % x != 0 {
                    continue;
                }

                for y in [x, nums[i] / x] {
                    if !range.contains_key(&y) {
                        range.insert(y, [i, i]);
                    }
                    range.get_mut(&y).unwrap()[1] = i;
                }
            }
        }

        for (&k, &v) in range.iter() {
            if k == 1 || v[0] == v[1] {
                continue;
            }

            arr[v[0]] += 1;
            arr[v[1]] -= 1;
        }

        for i in 0..n - 1 {
            prefix_sum += arr[i];

            if prefix_sum == 0 {
                return i as i32;
            }
        }

        -1
    }
}
```
