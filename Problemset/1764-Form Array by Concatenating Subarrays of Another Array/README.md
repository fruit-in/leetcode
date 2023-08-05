# 1764. Form Array by Concatenating Subarrays of Another Array
You are given a 2D integer array `groups` of length `n`. You are also given an integer array `nums`.

You are asked if you can choose `n` **disjoint** subarrays from the array `nums` such that the <code>i<sup>th</sup></code> subarray is equal to `groups[i]` (**0-indexed**), and if `i > 0`, the <code>(i-1)<sup>th</sup></code> subarray appears **before** the <code>i<sup>th</sup></code> subarray in `nums` (i.e. the subarrays must be in the same order as `groups`).

Return `true` *if you can do this task, and* `false` *otherwise*.

Note that the subarrays are **disjoint** if and only if there is no index `k` such that `nums[k]` belongs to more than one subarray. A subarray is a contiguous sequence of elements within an array.

#### Example 1:
<pre>
<strong>Input:</strong> groups = [[1,-1,-1],[3,-2,0]], nums = [1,-1,0,1,-1,-1,3,-2,0]
<strong>Output:</strong> true
<strong>Explanation:</strong> You can choose the 0th subarray as [1,-1,0,1,-1,-1,3,-2,0] and the 1st one as [1,-1,0,1,-1,-1,3,-2,0].
These subarrays are disjoint as they share no common nums[k] element.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> groups = [[10,-2],[1,2,3,4]], nums = [1,2,3,4,10,-2]
<strong>Output:</strong> false
<strong>Explanation:</strong> Note that choosing the subarrays [1,2,3,4,10,-2] and [1,2,3,4,10,-2] is incorrect because they are not in the same order as in groups.
[10,-2] must come before [1,2,3,4].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> groups = [[1,2,3],[3,4]], nums = [7,7,1,2,3,4,7,7]
<strong>Output:</strong> false
<strong>Explanation:</strong> Note that choosing the subarrays [7,7,1,2,3,4,7,7] and [7,7,1,2,3,4,7,7] is invalid because they are not disjoint.
They share a common elements nums[4] (0-indexed).
</pre>

#### Constraints:
* `groups.length == n`
* <code>1 <= n <= 10<sup>3</sup></code>
* <code>1 <= groups[i].length, sum(groups[i].length) <= 10<sup>3</sup></code>
* <code>1 <= nums.length <= 10<sup>3</sup></code>
* <code>-10<sup>7</sup> <= groups[i][j], nums[k] <= 10<sup>7</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
        let mut i = 0;

        for group in groups {
            let mut flag = true;

            loop {
                if i + group.len() > nums.len() {
                    return false;
                }

                let mut j = 0;

                while j < group.len() {
                    if nums[i + j] != group[j] {
                        flag = false;
                        break;
                    }
                    j += 1;
                }

                if flag {
                    i += j;
                    break;
                }

                i += 1;
                flag = true;
            }
        }

        true
    }
}
```
