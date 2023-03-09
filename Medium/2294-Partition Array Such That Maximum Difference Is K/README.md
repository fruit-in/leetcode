# 2294. Partition Array Such That Maximum Difference Is K
You are given an integer array `nums` and an integer `k`. You may partition `nums` into one or more **subsequences** such that each element in `nums` appears in **exactly** one of the subsequences.

Return *the **minimum** number of subsequences needed such that the difference between the maximum and minimum values in each subsequence is **at most*** `k`.

A **subsequence** is a sequence that can be derived from another sequence by deleting some or no elements without changing the order of the remaining elements.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,6,1,2,5], k = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong>
We can partition nums into the two subsequences [3,1,2] and [6,5].
The difference between the maximum and minimum value in the first subsequence is 3 - 1 = 2.
The difference between the maximum and minimum value in the second subsequence is 6 - 5 = 1.
Since two subsequences were created, we return 2. It can be shown that 2 is the minimum number of subsequences needed.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3], k = 1
<strong>Output:</strong> 2
<strong>Explanation:</strong>
We can partition nums into the two subsequences [1,2] and [3].
The difference between the maximum and minimum value in the first subsequence is 2 - 1 = 1.
The difference between the maximum and minimum value in the second subsequence is 3 - 3 = 0.
Since two subsequences were created, we return 2. Note that another optimal solution is to partition nums into the two subsequences [1] and [2,3].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [2,2,4,5], k = 0
<strong>Output:</strong> 3
<strong>Explanation:</strong>
We can partition nums into the three subsequences [2,2], [4], and [5].
The difference between the maximum and minimum value in the first subsequences is 2 - 2 = 0.
The difference between the maximum and minimum value in the second subsequences is 4 - 4 = 0.
The difference between the maximum and minimum value in the third subsequences is 5 - 5 = 0.
Since three subsequences were created, we return 3. It can be shown that 3 is the minimum number of subsequences needed.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>5</sup></code>
* <code>0 <= k <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let mut x = -k - 1;
        let mut ret = 0;
        nums.sort_unstable();

        for num in nums {
            if num - x > k {
                x = num;
                ret += 1;
            }
        }

        ret
    }
}
```
