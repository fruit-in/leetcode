# 2256. 最小平均差
给你一个下标从 **0** 开始长度为 `n` 的整数数组 `nums` 。

下标 `i` 处的 **平均差** 指的是 `nums` 中 **前** `i + 1` 个元素平均值和 **后** `n - i - 1` 个元素平均值的 **绝对差** 。两个平均值都需要 **向下取整** 到最近的整数。

请你返回产生 **最小平均差** 的下标。如果有多个下标最小平均差相等，请你返回 **最小** 的一个下标。

注意：

* 两个数的 **绝对差** 是两者差的绝对值。
* `n` 个元素的平均值是 `n` 个元素之 **和** 除以（整数除法） `n` 。
* `0` 个元素的平均值视为 `0` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,5,3,9,5,3]
<strong>输出:</strong> 3
<strong>解释:</strong>
- 下标 0 处的平均差为：|2 / 1 - (5 + 3 + 9 + 5 + 3) / 5| = |2 / 1 - 25 / 5| = |2 - 5| = 3 。
- 下标 1 处的平均差为：|(2 + 5) / 2 - (3 + 9 + 5 + 3) / 4| = |7 / 2 - 20 / 4| = |3 - 5| = 2 。
- 下标 2 处的平均差为：|(2 + 5 + 3) / 3 - (9 + 5 + 3) / 3| = |10 / 3 - 17 / 3| = |3 - 5| = 2 。
- 下标 3 处的平均差为：|(2 + 5 + 3 + 9) / 4 - (5 + 3) / 2| = |19 / 4 - 8 / 2| = |4 - 4| = 0 。
- 下标 4 处的平均差为：|(2 + 5 + 3 + 9 + 5) / 5 - 3 / 1| = |24 / 5 - 3 / 1| = |4 - 3| = 1 。
- 下标 5 处的平均差为：|(2 + 5 + 3 + 9 + 5 + 3) / 6 - 0| = |27 / 6 - 0| = |4 - 0| = 4 。
下标 3 处的平均差为最小平均差，所以返回 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [0]
<strong>输出:</strong> 0
<strong>解释:</strong>
唯一的下标是 0 ，所以我们返回 0 。
下标 0 处的平均差为：|0 / 1 - 0| = |0 - 0| = 0 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i64;
        let mut prefix_sum = 0;
        let mut suffix_sum = nums.iter().map(|&x| x as i64).sum::<i64>();
        let mut min_avg_diff = i64::MAX;
        let mut ret = 0;

        for i in 0..nums.len() {
            prefix_sum += nums[i] as i64;
            suffix_sum -= nums[i] as i64;
            let prefix_avg = prefix_sum / (i as i64 + 1);
            let suffix_avg = suffix_sum.checked_div(n - i as i64 - 1).unwrap_or(0);
            let avg_diff = (prefix_avg - suffix_avg).abs();

            if avg_diff < min_avg_diff {
                min_avg_diff = avg_diff;
                ret = i;
            }
        }

        ret as i32
    }
}
```
