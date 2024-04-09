# 891. 子序列宽度之和
一个序列的 **宽度** 定义为该序列中最大元素和最小元素的差值。

给你一个整数数组 `nums` ，返回 `nums` 的所有非空 **子序列** 的 **宽度之和** 。由于答案可能非常大，请返回对 <code>10<sup>9</sup> + 7</code> **取余** 后的结果。

**子序列** 定义为从一个数组里删除一些（或者不删除）元素，但不改变剩下元素的顺序得到的数组。例如，`[3,6,2,7]` 就是数组 `[0,3,1,6,2,2,7]` 的一个子序列。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,1,3]
<strong>输出:</strong> 6
<strong>解释:</strong> 子序列为 [1], [2], [3], [2,1], [2,3], [1,3], [2,1,3] 。
相应的宽度是 0, 0, 0, 1, 1, 2, 2 。
宽度之和是 6 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2]
<strong>输出:</strong> 0
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn sum_subseq_widths(mut nums: Vec<i32>) -> i32 {
        let mut x = 0;
        let mut pow2 = 1;
        let mut pow2_sum = 1;
        let mut ret = 0;

        nums.sort_unstable();

        for i in 1..nums.len() {
            x = (2 * x + (nums[i] - nums[i - 1]) as i64 * pow2_sum) % 1_000_000_007;
            pow2 = (2 * pow2) % 1_000_000_007;
            pow2_sum = (pow2_sum + pow2) % 1_000_000_007;
            ret = (ret + x) % 1_000_000_007;
        }

        ret as i32
    }
}
```
