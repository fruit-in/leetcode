# 2090. 半径为 k 的子数组平均值
给你一个下标从 **0** 开始的数组 `nums` ，数组中有 `n` 个整数，另给你一个整数 `k` 。

**半径为 k 的子数组平均值** 是指：`nums` 中一个以下标 `i` 为 **中心** 且 **半径** 为 `k` 的子数组中所有元素的平均值，即下标在 `i - k` 和 `i + k` 范围（**含** `i - k` 和 `i + k`）内所有元素的平均值。如果在下标 `i` 前或后不足 `k` 个元素，那么 **半径为 k 的子数组平均值** 是 `-1` 。

构建并返回一个长度为 `n` 的数组 `avgs` ，其中 `avgs[i]` 是以下标 `i` 为中心的子数组的 **半径为 k 的子数组平均值** 。

`x` 个元素的 **平均值** 是 `x` 个元素相加之和除以 `x` ，此时使用截断式 **整数除法** ，即需要去掉结果的小数部分。

* 例如，四个元素 `2`、`3`、`1` 和 `5` 的平均值是 `(2 + 3 + 1 + 5) / 4 = 11 / 4 = 2.75`，截断后得到 `2` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/11/07/eg1.png)
<pre>
<strong>输入:</strong> nums = [7,4,3,9,1,8,5,2,6], k = 3
<strong>输出:</strong> [-1,-1,-1,5,4,4,-1,-1,-1]
<strong>解释:</strong>
- avg[0]、avg[1] 和 avg[2] 是 -1 ，因为在这几个下标前的元素数量都不足 k 个。
- 中心为下标 3 且半径为 3 的子数组的元素总和是：7 + 4 + 3 + 9 + 1 + 8 + 5 = 37 。
  使用截断式 整数除法，avg[3] = 37 / 7 = 5 。
- 中心为下标 4 的子数组，avg[4] = (4 + 3 + 9 + 1 + 8 + 5 + 2) / 7 = 4 。
- 中心为下标 5 的子数组，avg[5] = (3 + 9 + 1 + 8 + 5 + 2 + 6) / 7 = 4 。
- avg[6]、avg[7] 和 avg[8] 是 -1 ，因为在这几个下标后的元素数量都不足 k 个。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [100000], k = 0
<strong>输出:</strong> [100000]
<strong>解释:</strong>
- 中心为下标 0 且半径 0 的子数组的元素总和是：100000 。
  avg[0] = 100000 / 1 = 100000 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [8], k = 100000
<strong>输出:</strong> [-1]
<strong>解释:</strong>
- avg[0] 是 -1 ，因为在下标 0 前后的元素数量均不足 k 。
</pre>

#### 提示:
* `n == nums.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>0 <= nums[i], k <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut avgs = vec![-1; nums.len()];
        let mut sum = nums.iter().take(2 * k + 1).map(|&x| x as i64).sum::<i64>();

        for i in k..nums.len().saturating_sub(k) {
            avgs[i] = (sum / (2 * k as i64 + 1)) as i32;
            sum += (nums.get(i + k + 1).unwrap_or(&0) - nums[i - k]) as i64;
        }

        avgs
    }
}
```
