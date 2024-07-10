# 1856. 子数组最小乘积的最大值
一个数组的 **最小乘积** 定义为这个数组中 **最小值** **乘以** 数组的 **和** 。

* 比方说，数组 `[3,2,5]` （最小值是 `2`）的最小乘积为 `2 * (3+2+5) = 2 * 10 = 20` 。

给你一个正整数数组 `nums` ，请你返回 `nums` 任意 **非空子数组** 的**最小乘积** 的 **最大值** 。由于答案可能很大，请你返回答案对  <code>10<sup>9</sup> + 7</code> **取余** 的结果。

请注意，最小乘积的最大值考虑的是取余操作 **之前** 的结果。题目保证最小乘积的最大值在 **不取余** 的情况下可以用 **64 位有符号整数** 保存。

**子数组** 定义为一个数组的 **连续** 部分。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,2]
<strong>输出:</strong> 14
<strong>解释:</strong> 最小乘积的最大值由子数组 [2,3,2] （最小值是 2）得到。
2 * (2+3+2) = 2 * 7 = 14 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,3,3,1,2]
<strong>输出:</strong> 18
<strong>解释:</strong> 最小乘积的最大值由子数组 [3,3] （最小值是 3）得到。
3 * (3+3) = 3 * 6 = 18 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [3,1,5,6,4,2]
<strong>输出:</strong> 60
<strong>解释:</strong> 最小乘积的最大值由子数组 [5,6,4] （最小值是 4）得到。
4 * (5+6+4) = 4 * 15 = 60 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>7</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_sum_min_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut prefix_sum = vec![0; n + 1];
        let mut stack = vec![];
        let mut indexl = vec![None; n];
        let mut indexr = vec![None; n];
        let mut ret = nums[0] as i64 * nums[0] as i64;

        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i] as i64;

            while let Some(&j) = stack.last() {
                if nums[j] >= nums[i] {
                    stack.pop();
                } else {
                    indexl[i] = Some(j);
                    break;
                }
            }
            stack.push(i);
        }

        stack.clear();
        for i in (0..n).rev() {
            while let Some(&j) = stack.last() {
                if nums[j] >= nums[i] {
                    stack.pop();
                } else {
                    indexr[i] = Some(j);
                    break;
                }
            }
            stack.push(i);

            let mut sum = prefix_sum[indexr[i].unwrap_or(n)];
            if let Some(j) = indexl[i] {
                sum -= prefix_sum[j + 1];
            }
            ret = ret.max(nums[i] as i64 * sum);
        }

        (ret % 1_000_000_007) as i32
    }
}
```
