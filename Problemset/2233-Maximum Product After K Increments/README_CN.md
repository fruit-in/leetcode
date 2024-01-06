# 2233. K 次增加后的最大乘积
给你一个非负整数数组 `nums` 和一个整数 `k` 。每次操作，你可以选择 `nums` 中 **任一** 元素并将它 **增加** `1` 。

请你返回 **至多** `k` 次操作后，能得到的 `nums`的 **最大乘积** 。由于答案可能很大，请你将答案对 <code>10<sup>9</sup> + 7</code> 取余后返回。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [0,4], k = 5
<strong>输出:</strong> 20
<strong>解释:</strong> 将第一个数增加 5 次。
得到 nums = [5, 4] ，乘积为 5 * 4 = 20 。
可以证明 20 是能得到的最大乘积，所以我们返回 20 。
存在其他增加 nums 的方法，也能得到最大乘积。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [6,3,3,2], k = 2
<strong>输出:</strong> 216
<strong>解释:</strong> 将第二个数增加 1 次，将第四个数增加 1 次。
得到 nums = [6, 4, 3, 3] ，乘积为 6 * 4 * 3 * 3 = 216 。
可以证明 216 是能得到的最大乘积，所以我们返回 216 。
存在其他增加 nums 的方法，也能得到最大乘积。
</pre>

#### 提示:
* <code>1 <= nums.length, k <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums.into_iter().map(|num| -num).collect::<BinaryHeap<_>>();

        for _ in 0..k {
            *nums.peek_mut().unwrap() -= 1;
        }

        nums.iter()
            .fold(1, |acc, x| acc * (-x as i64) % 1_000_000_007) as i32
    }
}
```
