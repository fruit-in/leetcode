# 2091. Removing Minimum and Maximum From Array
You are given a **0-indexed** array of **distinct** integers `nums`.

There is an element in `nums` that has the **lowest** value and an element that has the **highest** value. We call them the **minimum** and **maximum** respectively. Your goal is to remove **both** these elements from the array.

A **deletion** is defined as either removing an element from the **front** of the array or removing an element from the **back** of the array.

Return *the **minimum** number of deletions it would take to remove **both** the minimum and maximum element from the array*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,10,7,5,4,1,8,6]
<strong>Output:</strong> 5
<strong>Explanation:</strong>
The minimum element in the array is nums[5], which is 1.
The maximum element in the array is nums[1], which is 10.
We can remove both the minimum and maximum by removing 2 elements from the front and 3 elements from the back.
This results in 2 + 3 = 5 deletions, which is the minimum number possible.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [0,-4,19,1,8,-2,-3,5]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
The minimum element in the array is nums[1], which is -4.
The maximum element in the array is nums[2], which is 19.
We can remove both the minimum and maximum by removing 3 elements from the front.
This results in only 3 deletions, which is the minimum number possible.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [101]
<strong>Output:</strong> 1
<strong>Explanation:</strong>
There is only one element in the array, which makes it both the minimum and maximum element.
We can remove it with 1 deletion.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup></code>
* The integers in `nums` are **distinct**.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let min_index = nums.iter().enumerate().min_by_key(|(_, &x)| x).unwrap().0;
        let max_index = nums.iter().enumerate().max_by_key(|(_, &x)| x).unwrap().0;
        let left = min_index.min(max_index);
        let right = min_index.max(max_index);

        (right + 1)
            .min(nums.len() - left)
            .min(left + 1 + nums.len() - right) as i32
    }
}
```
