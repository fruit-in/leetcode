# 2448. 使数组相等的最小开销
给你两个下标从 **0** 开始的数组 `nums` 和 `cost` ，分别包含 `n` 个 **正** 整数。

你可以执行下面操作 **任意** 次：
* 将 `nums` 中 **任意** 元素增加或者减小 `1` 。

对第 `i` 个元素执行一次操作的开销是 `cost[i]` 。

请你返回使 `nums` 中所有元素 **相等** 的 **最少** 总开销。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,3,5,2], cost = [2,3,1,14]
<strong>输出:</strong> 8
<strong>解释:</strong> 我们可以执行以下操作使所有元素变为 2 ：
- 增加第 0 个元素 1 次，开销为 2 。
- 减小第 1 个元素 1 次，开销为 3 。
- 减小第 2 个元素 3 次，开销为 1 + 1 + 1 = 3 。
总开销为 2 + 3 + 3 = 8 。
这是最小开销。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,2,2,2,2], cost = [4,2,8,1,3]
<strong>输出:</strong> 0
<strong>解释:</strong> 数组中所有元素已经全部相等，不需要执行额外的操作。
</pre>

#### 提示:
* `n == nums.length == cost.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= nums[i], cost[i] <= 10<sup>6</sup></code>
* 测试用例确保输出不超过 2<sup>53</sup>-1。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        let mut nums = (0..nums.len())
            .map(|i| (nums[i] as i64, cost[i] as i64))
            .collect::<Vec<_>>();
        let mut prefix_cost = 0;
        let mut suffix_cost = 0;
        let mut equal_cost = 0;
        let mut total_cost = 0;
        let mut i = 0;
        let mut ret = i64::MAX;

        nums.sort_unstable();

        for j in 0..nums.len() {
            suffix_cost += nums[j].1;
            total_cost += nums[j].1 * (nums[j].0 - nums[0].0 + 1);
        }

        ret = ret.min(total_cost);

        for x in nums[0].0..=nums[nums.len() - 1].0 {
            prefix_cost += equal_cost;
            equal_cost = 0;
            total_cost += prefix_cost - suffix_cost;
            ret = ret.min(total_cost);
            while i < nums.len() && nums[i].0 == x {
                suffix_cost -= nums[i].1;
                equal_cost += nums[i].1;
                i += 1;
            }
        }

        ret
    }
}
```
