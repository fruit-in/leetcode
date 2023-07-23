# 2239. Find Closest Number to Zero
Given an integer array `nums` of size `n`, return *the number with the value **closest** to* `0` *in* `nums`. If there are multiple answers, return *the number with the **largest** value*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [-4,-2,1,4,8]
<strong>Output:</strong> 1
<strong>Explanation:</strong>
The distance from -4 to 0 is |-4| = 4.
The distance from -2 to 0 is |-2| = 2.
The distance from 1 to 0 is |1| = 1.
The distance from 4 to 0 is |4| = 4.
The distance from 8 to 0 is |8| = 8.
Thus, the closest number to 0 in the array is 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,-1,1]
<strong>Output:</strong> 1
<strong>Explanation:</strong> 1 and -1 are both the closest numbers to 0, so 1 being larger is returned.
</pre>

#### Constraints:
* `1 <= n <= 1000`
* <code>-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .min_by_key(|num| (num.abs(), -num))
            .unwrap()
    }
}
```
