# 330. Patching Array
Given a sorted integer array `nums` and an integer `n`, add/patch elements to the array such that any number in the range `[1, n]` inclusive can be formed by the sum of some elements in the array.

Return *the minimum number of patches required*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,3], n = 6
<strong>Output:</strong> 1
<strong>Explanation:</strong>
Combinations of nums are [1], [3], [1,3], which form possible sums of: 1, 3, 4.
Now if we add/patch 2 to nums, the combinations are: [1], [2], [3], [1,3], [2,3], [1,2,3].
Possible sums are 1, 2, 3, 4, 5, 6, which now covers the range [1, 6].
So we only need 1 patch.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,5,10], n = 20
<strong>Output:</strong> 2
<strong>Explanation:</strong> The two patches can be [2, 4].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,2,2], n = 5
<strong>Output:</strong> 0
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* <code>1 <= nums[i] <= 10<sup>4</sup></code>
* `nums` is sorted in ascending order.
* <code>1 <= n <= 2<sup>31</sup> - 1</code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let nums = nums.into_iter().map(|num| num as i64).collect::<Vec<_>>();
        let n = n as i64;
        let mut sum = 0;
        let mut ret = 0;

        for &num in &nums {
            while sum < num - 1 && sum < n {
                sum += sum + 1;
                ret += 1;
            }
            sum += num;

            if sum >= n {
                break;
            }
        }

        while sum < n {
            sum += sum + 1;
            ret += 1;
        }

        ret
    }
}
```
