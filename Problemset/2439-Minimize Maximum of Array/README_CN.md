# 2439. 最小化数组中的最大值
给你一个下标从 **0** 开始的数组 `nums` ，它含有 `n` 个非负整数。

每一步操作中，你需要：

* 选择一个满足 `1 <= i < n` 的整数 `i` ，且 `nums[i] > 0` 。
* 将 `nums[i]` 减 1 。
* 将 `nums[i - 1]` 加 1 。

你可以对数组执行 **任意** 次上述操作，请你返回可以得到的 `nums` 数组中 **最大值** **最小** 为多少。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,7,1,6]
<strong>输出:</strong> 5
<strong>解释:</strong>
一串最优操作是：
1. 选择 i = 1 ，nums 变为 [4,6,1,6] 。
2. 选择 i = 3 ，nums 变为 [4,6,2,5] 。
3. 选择 i = 1 ，nums 变为 [5,5,2,5] 。
nums 中最大值为 5 。无法得到比 5 更小的最大值。
所以我们返回 5 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [10,1]
<strong>输出:</strong> 10
<strong>解释:</strong>
最优解是不改动 nums ，10 是最大值，所以返回 10 。
</pre>

#### 提示:
* `n == nums.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut lo = *nums.iter().min().unwrap() as i64;
        let mut hi = *nums.iter().max().unwrap() as i64;

        while lo < hi {
            let m = (lo + hi) / 2;
            let mut x = nums[n - 1] as i64;

            for i in (1..n).rev() {
                if x > m {
                    x += nums[i - 1] as i64 - m;
                } else {
                    x = nums[i - 1] as i64;
                }
            }

            if x > m {
                lo = m + 1;
            } else {
                hi = m;
            }
        }

        hi as i32
    }
}
```
