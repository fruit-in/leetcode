# 918. 环形子数组的最大和
给定一个长度为 `n` 的**环形整数数组** `nums` ，返回 *`nums` 的非空 **子数组** 的最大可能和* 。

**环形数组** 意味着数组的末端将会与开头相连呈环状。形式上， `nums[i]` 的下一个元素是 `nums[(i + 1) % n]` ， `nums[i]` 的前一个元素是 `nums[(i - 1 + n) % n]` 。

**子数组** 最多只能包含固定缓冲区 `nums` 中的每个元素一次。形式上，对于子数组 `nums[i], nums[i + 1], ..., nums[j]` ，不存在 `i <= k1, k2 <= j` 其中 `k1 % n == k2 % n` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,-2,3,-2]
<strong>输出:</strong> 3
<strong>解释:</strong> 从子数组 [3] 得到最大和 3
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [5,-3,5]
<strong>输出:</strong> 10
<strong>解释:</strong> 从子数组 [5,5] 得到最大和 5 + 5 = 10
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [3,-2,2,-3]
<strong>输出:</strong> 3
<strong>解释:</strong> 从子数组 [3] 和 [3,-2,2] 都可以得到最大和 3
</pre>

#### 提示:
* `n == nums.length`
* <code>1 <= n <= 3 * 10<sup>4</sup></code>
* <code>-3 * 10<sup>4</sup> <= nums[i] <= 3 * 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().sum();
        let mut max_sum = nums[0];
        let mut min_sum = nums[0];
        let mut prefix_sum = nums[0];
        let mut ret = nums[0].max(sum);

        for i in 1..nums.len() {
            prefix_sum += nums[i];
            ret = ret
                .max(prefix_sum - min_sum)
                .max(sum - prefix_sum + max_sum);
            max_sum = max_sum.max(prefix_sum);
            min_sum = min_sum.min(prefix_sum);
        }

        ret
    }
}
```
