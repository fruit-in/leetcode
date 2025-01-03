# 128. 最长连续序列
给定一个未排序的整数数组 `nums` ，找出数字连续的最长序列（不要求序列元素在原数组中连续）的长度。

请你设计并实现时间复杂度为 `O(n)` 的算法解决此问题。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [100,4,200,1,3,2]
<strong>输出:</strong> 4
<strong>解释:</strong> 最长数字连续序列是 [1, 2, 3, 4]。它的长度为 4。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [0,3,7,2,5,8,4,6,0,1]
<strong>输出:</strong> 9
</pre>

#### 提示:
* <code>0 <= nums.length <= 10<sup>5</sup></code>
* <code>-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut leftmost = HashMap::new();
        let mut rightmost = HashMap::new();
        let mut ret = 0;

        for &&x in nums.iter().collect::<HashSet<_>>().iter() {
            let lo = leftmost.remove(&(x - 1)).unwrap_or(x);
            let hi = rightmost.remove(&(x + 1)).unwrap_or(x);

            leftmost.insert(hi, lo);
            rightmost.insert(lo, hi);

            ret = ret.max(hi - lo + 1);
        }

        ret
    }
}
```
