# 1675. Minimize Deviation in Array
You are given an array `nums` of `n` positive integers.

You can perform two types of operations on any element of the array any number of times:
* If the element is **even**, **divide** it by `2`.
    * For example, if the array is `[1,2,3,4]`, then you can do this operation on the last element, and the array will be `[1,2,3,2]`.
* If the element is **odd**, **multiply** it by `2`.
    * For example, if the array is `[1,2,3,4]`, then you can do this operation on the first element, and the array will be `[2,2,3,4]`.

The **deviation** of the array is the **maximum difference** between any two elements in the array.

Return *the **minimum deviation** the array can have after performing some number of operations*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3,4]
<strong>Output:</strong> 1
<strong>Explanation:</strong> You can transform the array to [1,2,3,2], then to [2,2,3,2], then the deviation will be 3 - 2 = 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [4,1,5,20,3]
<strong>Output:</strong> 3
<strong>Explanation:</strong> You can transform the array after two operations to [4,2,5,5,3], then the deviation will be 5 - 2 = 3.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [2,10,8]
<strong>Output:</strong> 3
</pre>

#### Constraints:
* `n == nums.length`
* <code>2 <= n <= 5 * 10<sup>4</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
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
