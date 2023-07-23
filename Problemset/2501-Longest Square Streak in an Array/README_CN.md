# 2501. 数组中最长的方波
给你一个整数数组 `nums` 。如果 `nums` 的子序列满足下述条件，则认为该子序列是一个 **方波** ：

* 子序列的长度至少为 `2` ，并且
* 将子序列从小到大排序 **之后** ，除第一个元素外，每个元素都是前一个元素的 **平方** 。

返回 `nums` 中 **最长方波** 的长度，如果不存在 **方波** 则返回 `-1` 。

**子序列** 也是一个数组，可以由另一个数组删除一些或不删除元素且不改变剩余元素的顺序得到。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [4,3,6,16,8,2]
<strong>输出:</strong> 3
<strong>解释:</strong> 选出子序列 [4,16,2] 。排序后，得到 [2,4,16] 。
- 4 = 2 * 2.
- 16 = 4 * 4.
因此，[4,16,2] 是一个方波.
可以证明长度为 4 的子序列都不是方波。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,3,5,6,7]
<strong>输出:</strong> -1
<strong>解释:</strong> nums 不存在方波，所以返回 -1 。
</pre>

#### 提示:
* <code>2 <= nums.length <= 10<sup>5</sup></code>
* <code>2 <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut nums_set = nums.clone().into_iter().collect::<HashSet<_>>();
        let mut ret = -1;

        nums.sort_unstable();

        for &num in &nums {
            let mut length = 1;
            let mut x = num;

            while x < 317 && nums_set.remove(&(x * x)) {
                x *= x;
                length += 1;
            }

            if length > ret.max(1) {
                ret = length
            }
        }

        ret
    }
}
```
