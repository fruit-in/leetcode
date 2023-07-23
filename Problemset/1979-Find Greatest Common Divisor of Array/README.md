# 1979. Find Greatest Common Divisor of Array
Given an integer array `nums`, return *the **greatest common divisor** of the smallest number and largest number in* `nums`.

The **greatest common divisor** of two numbers is the largest positive integer that evenly divides both numbers.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,5,6,9,10]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
The smallest number in nums is 2.
The largest number in nums is 10.
The greatest common divisor of 2 and 10 is 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [7,5,6,8,3]
<strong>Output:</strong> 1
<strong>Explanation:</strong>
The smallest number in nums is 3.
The largest number in nums is 8.
The greatest common divisor of 3 and 8 is 1.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [3,3]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
The smallest number in nums is 3.
The largest number in nums is 3.
The greatest common divisor of 3 and 3 is 3.
</pre>

#### Constraints:
* `2 <= nums.length <= 1000`
* `1 <= nums[i] <= 1000`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def findGCD(self, nums: List[int]) -> int:
        return gcd(min(nums), max(nums))
```

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def find_gcd(nums)
  nums.min.gcd(nums.max)
end
```
