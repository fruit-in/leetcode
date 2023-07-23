# 1295. Find Numbers with Even Number of Digits
Given an array ```nums``` of integers, return how many of them contain an **even number** of digits.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [12,345,2,6,7896]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
12 contains 2 digits (even number of digits).
345 contains 3 digits (odd number of digits).
2 contains 1 digit (odd number of digits).
6 contains 1 digit (odd number of digits).
7896 contains 4 digits (even number of digits).
Therefore only 12 and 7896 contain an even number of digits.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [555,901,482,1771]
<strong>Output:</strong> 1
<strong>Explanation:</strong>
Only 1771 contains an even number of digits.
</pre>

#### Constraints:
* ```1 <= nums.length <= 500```
* ```1 <= nums[i] <= 10^5```

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def findNumbers(self, nums: List[int]) -> int:
        return sum(1 for n in nums if len(str(n)) % 2 == 0)
```
