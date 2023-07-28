# 2134. Minimum Swaps to Group All 1's Together II
A **swap** is defined as taking two **distinct** positions in an array and swapping the values in them.

A **circular** array is defined as an array where we consider the **first** element and the **last** element to be **adjacent**.

Given a **binary circular** array `nums`, return *the minimum number of swaps required to group all* `1`*'s present in the array together at **any location***.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [0,1,0,1,1,0,0]
<strong>Output:</strong> 1
<strong>Explanation:</strong> Here are a few of the ways to group all the 1's together:
[0,0,1,1,1,0,0] using 1 swap.
[0,1,1,1,0,0,0] using 1 swap.
[1,1,0,0,0,0,1] using 2 swaps (using the circular property of the array).
There is no way to group all 1's together with 0 swaps.
Thus, the minimum number of swaps required is 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [0,1,1,1,0,0,1,1,0]
<strong>Output:</strong> 2
<strong>Explanation:</strong> Here are a few of the ways to group all the 1's together:
[1,1,1,0,0,0,0,1,1] using 2 swaps (using the circular property of the array).
[1,1,1,1,1,0,0,0,0] using 2 swaps.
There is no way to group all 1's together with 0 or 1 swaps.
Thus, the minimum number of swaps required is 2.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,1,0,0,1]
<strong>Output:</strong> 0
<strong>Explanation:</strong> All the 1's are already grouped together due to the circular property of the array.
Thus, the minimum number of swaps required is 0.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* `nums[i]` is either `0` or `1`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let count_1 = nums.iter().filter(|&&x| x == 1).count();
        let mut count_0 = nums.iter().take(count_1).filter(|&&x| x == 0).count();
        let mut ret = count_0;

        for i in 0..nums.len() - 1 {
            count_0 -= (nums[i] == 0) as usize;
            count_0 += (nums[(i + count_1) % nums.len()] == 0) as usize;
            ret = ret.min(count_0);
        }

        ret as i32
    }
}
```
