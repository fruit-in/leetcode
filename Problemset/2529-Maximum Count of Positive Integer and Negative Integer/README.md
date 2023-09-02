# 2529. Maximum Count of Positive Integer and Negative Integer
Given an array `nums` sorted in **non-decreasing** order, return *the maximum between the number of positive integers and the number of negative integers*.

* In other words, if the number of positive integers in `nums` is `pos` and the number of negative integers is `neg`, then return the maximum of `pos` and `neg`.

**Note** that `0` is neither positive nor negative.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [-2,-1,-1,1,2,3]
<strong>Output:</strong> 3
<strong>Explanation:</strong> There are 3 positive integers and 3 negative integers. The maximum count among them is 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [-3,-2,-1,0,0,1,2]
<strong>Output:</strong> 3
<strong>Explanation:</strong> There are 2 positive integers and 3 negative integers. The maximum count among them is 3.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [5,20,66,1314]
<strong>Output:</strong> 4
<strong>Explanation:</strong> There are 4 positive integers and 0 negative integers. The maximum count among them is 4.
</pre>

#### Constraints:
* `1 <= nums.length <= 2000`
* `-2000 <= nums[i] <= 2000`
* `nums` is sorted in a **non-decreasing order**.

**Follow up:** Can you solve the problem in `O(log(n))` time complexity?

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def maximumCount(self, nums: List[int]) -> int:
        pos = bisect.bisect_left(nums, 0)
        neg = len(nums) - bisect.bisect_right(nums, 0)

        return max(pos, neg)
```
