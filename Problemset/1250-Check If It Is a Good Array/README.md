# 1250. Check If It Is a Good Array
Given an array `nums` of positive integers. Your task is to select some subset of `nums`, multiply each element by an integer and add all these numbers. The array is said to be **good** if you can obtain a sum of `1` from the array by any possible subset and multiplicand.

Return `True` if the array is **good** otherwise return `False`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [12,5,7,23]
<strong>Output:</strong> true
<strong>Explanation:</strong> Pick numbers 5 and 7.
5*3 + 7*(-2) = 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [29,6,10]
<strong>Output:</strong> true
<strong>Explanation:</strong> Pick numbers 29, 6 and 10.
29*1 + 6*(-3) + 10*(-1) = 1
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [3,6]
<strong>Output:</strong> false
</pre>

#### Constraints:
* `1 <= nums.length <= 10^5`
* `1 <= nums[i] <= 10^9`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def isGoodArray(self, nums: List[int]) -> bool:
        return math.gcd(*nums) == 1
```
