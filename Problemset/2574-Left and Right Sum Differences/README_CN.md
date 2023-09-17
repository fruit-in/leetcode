# 2574. 左右元素和的差值
给你一个下标从 **0** 开始的整数数组 `nums` ，请你找出一个下标从 **0** 开始的整数数组 `answer` ，其中：

* `answer.length == nums.length`
* `answer[i] = |leftSum[i] - rightSum[i]|`

其中：

* `leftSum[i]` 是数组 `nums` 中下标 `i` 左侧元素之和。如果不存在对应的元素，`leftSum[i] = 0` 。
* `rightSum[i]` 是数组 `nums` 中下标 `i` 右侧元素之和。如果不存在对应的元素，`rightSum[i] = 0` 。

返回数组 `answer` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [10,4,8,3]
<strong>输出:</strong> [15,1,11,22]
<strong>解释:</strong> 数组 leftSum 为 [0,10,14,22] 且数组 rightSum 为 [15,11,3,0] 。
数组 answer 为 [|0 - 15|,|10 - 11|,|14 - 3|,|22 - 0|] = [15,1,11,22] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1]
<strong>输出:</strong> [0]
<strong>解释:</strong> 数组 leftSum 为 [0] 且数组 rightSum 为 [0] 。
数组 answer 为 [|0 - 0|] = [0] 。
</pre>

#### 提示:
* `1 <= nums.length <= 1000`
* <code>1 <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut left_sum = vec![0; n];
        let mut right_sum = vec![0; n];

        for i in 1..n {
            left_sum[i] = left_sum[i - 1] + nums[i - 1];
            right_sum[n - 1 - i] = right_sum[n - i] + nums[n - i];
        }

        (0..n).map(|i| (left_sum[i] - right_sum[i]).abs()).collect()
    }
}
```
