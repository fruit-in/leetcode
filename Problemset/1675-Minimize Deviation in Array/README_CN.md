# 1675. 数组的最小偏移量
给你一个由 `n` 个正整数组成的数组 `nums` 。

你可以对数组的任意元素执行任意次数的两类操作：
* 如果元素是 **偶数** ，**除以** `2`
    * 例如，如果数组是 `[1,2,3,4]` ，那么你可以对最后一个元素执行此操作，使其变成 `[1,2,3,2]`
* 如果元素是 **奇数** ，**乘上** `2`
    * 例如，如果数组是 `[1,2,3,4]` ，那么你可以对第一个元素执行此操作，使其变成 `[2,2,3,4]`

数组的 **偏移量** 是数组中任意两个元素之间的 **最大差值** 。

返回数组在执行某些操作之后可以拥有的 **最小偏移量** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,4]
<strong>输出:</strong> 1
<strong>解释:</strong> 你可以将数组转换为 [1,2,3,2]，然后转换成 [2,2,3,2]，偏移量是 3 - 2 = 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [4,1,5,20,3]
<strong>输出:</strong> 3
<strong>解释:</strong> 两次操作后，你可以将数组转换为 [4,2,5,5,3]，偏移量是 5 - 2 = 3
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [2,10,8]
<strong>输出:</strong> 3
</pre>

#### 提示:
* `n == nums.length`
* <code>2 <= n <= 5 * 10<sup>4</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut min_num = i32::MAX;
        let mut ret = i32::MAX;

        for &num in &nums {
            heap.push(num * (num % 2 + 1));
            min_num = min_num.min(num * (num % 2 + 1));
        }

        while let Some(num) = heap.pop() {
            ret = ret.min(num - min_num);

            if num % 2 == 0 {
                heap.push(num / 2);
                min_num = min_num.min(num / 2);
            } else {
                break;
            }
        }

        ret
    }
}
```
