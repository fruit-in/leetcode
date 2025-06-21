# 2530. 执行 K 次操作后的最大分数
给你一个下标从 **0** 开始的整数数组 `nums` 和一个整数 `k` 。你的 **起始分数** 为 `0` 。

在一步 **操作** 中：
1. 选出一个满足 `0 <= i < nums.length` 的下标 `i` ，
2. 将你的 **分数** 增加 `nums[i]` ，并且
3. 将 `nums[i]` 替换为 `ceil(nums[i] / 3)` 。

返回在 **恰好** 执行 `k` 次操作后，你可能获得的最大分数。

向上取整函数 `ceil(val)` 的结果是大于或等于 `val` 的最小整数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [10,10,10,10,10], k = 5
<strong>输出:</strong> 50
<strong>解释:</strong> 对数组中每个元素执行一次操作。最后分数是 10 + 10 + 10 + 10 + 10 = 50 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,10,3,3,3], k = 3
<strong>输出:</strong> 17
<strong>解释:</strong> 可以执行下述操作：
第 1 步操作：选中 i = 1 ，nums 变为 [1,4,3,3,3] 。分数增加 10 。
第 2 步操作：选中 i = 1 ，nums 变为 [1,2,3,3,3] 。分数增加 4 。
第 3 步操作：选中 i = 2 ，nums 变为 [1,2,1,3,3] 。分数增加 3 。
最后分数是 10 + 4 + 3 = 17 。
</pre>

#### 提示:
* <code>1 <= nums.length, k <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums.iter().map(|&x| x as i64).collect::<BinaryHeap<_>>();
        let mut ret = 0;

        for _ in 0..k {
            let x = nums.pop().unwrap();
            nums.push((x + 2) / 3);
            ret += x;
        }

        ret
    }
}
```
