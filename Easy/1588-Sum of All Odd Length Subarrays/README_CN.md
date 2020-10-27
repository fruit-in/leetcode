# 1588. 所有奇数长度子数组的和
给你一个正整数数组 `arr` ，请你计算所有可能的奇数长度子数组的和。

**子数组** 定义为原数组中的一个连续子序列。

请你返回 `arr` 中 **所有奇数长度子数组的和** 。

#### 示例 1:
<pre>
<b>输入:</b> arr = [1,4,2,5,3]
<b>输出:</b> 58
<b>解释:</b> 所有奇数长度子数组和它们的和为：
[1] = 1
[4] = 4
[2] = 2
[5] = 5
[3] = 3
[1,4,2] = 7
[4,2,5] = 11
[2,5,3] = 10
[1,4,2,5,3] = 15
我们将所有值求和得到 1 + 4 + 2 + 5 + 3 + 7 + 11 + 10 + 15 = 58
</pre>

#### 示例 2:
<pre>
<b>输入:</b> arr = [1,2]
<b>输出:</b> 3
<b>解释:</b> 总共只有 2 个长度为奇数的子数组，[1] 和 [2]。它们的和为 3 。
</pre>

#### 示例 3:
<pre>
<b>输入:</b> arr = [10,11,12]
<b>输出:</b> 66
</pre>

#### 提示:
* `1 <= arr.length <= 100`
* `1 <= arr[i] <= 1000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut d = (arr.len() as i32 + 1) / 2;
        let mut prev = d;
        let mut ret = 0;

        for i in 0..(arr.len() / 2) {
            let j = arr.len() - 1 - i;

            ret += (arr[i] + arr[j]) * prev;
            d -= match arr.len() % 2 {
                0 => 1,
                _ => 2 * (1 - i as i32 % 2),
            };
            prev += d;
        }

        if arr.len() % 2 == 1 {
            ret += arr[arr.len() / 2] * prev;
        }

        ret
    }
}
```
