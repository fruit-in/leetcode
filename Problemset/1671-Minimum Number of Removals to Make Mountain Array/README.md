# 1671. Minimum Number of Removals to Make Mountain Array
You may recall that an array `arr` is a **mountain array** if and only if:
* `arr.length >= 3`
* There exists some index `i` (**0-indexed**) with `0 < i < arr.length - 1` such that:
    * `arr[0] < arr[1] < ... < arr[i - 1] < arr[i]`
    * `arr[i] > arr[i + 1] > ... > arr[arr.length - 1]`

Given an integer array `nums`, return *the **minimum** number of elements to remove to make* `nums` *a **mountain array***.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,3,1]
<strong>Output:</strong> 0
<strong>Explanation:</strong> The array itself is a mountain array so we do not need to remove any elements.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,1,1,5,6,2,3,1]
<strong>Output:</strong> 3
<strong>Explanation:</strong> One solution is to remove the elements at indices 0, 1, and 5, making the array nums = [1,5,6,3,1].
</pre>

#### Constraints:
* `3 <= nums.length <= 1000`
* <code>1 <= nums[i] <= 10<sup>9</sup></code>
* It is guaranteed that you can make a mountain array out of `nums`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let mut lis_last = vec![0];
        let mut lis_left = vec![0; nums.len()];
        let mut lis_right = vec![0; nums.len()];

        for i in 0..nums.len() {
            let j = match lis_last.binary_search(&nums[i]) {
                Ok(j) => j,
                Err(j) if j == lis_last.len() => {
                    lis_last.push(0);
                    j
                }
                Err(j) => j,
            };

            lis_last[j] = nums[i];
            lis_left[i] = j;
        }

        lis_last = vec![0];
        for i in (0..nums.len()).rev() {
            let j = match lis_last.binary_search(&nums[i]) {
                Ok(j) => j,
                Err(j) if j == lis_last.len() => {
                    lis_last.push(0);
                    j
                }
                Err(j) => j,
            };

            lis_last[j] = nums[i];
            lis_right[i] = j;
        }

        (0..nums.len())
            .filter(|&i| lis_left[i] > 1 && lis_right[i] > 1)
            .map(|i| nums.len() + 1 - lis_left[i] - lis_right[i])
            .min()
            .unwrap() as i32
    }
}
```
