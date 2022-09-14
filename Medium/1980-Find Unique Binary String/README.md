# 1980. Find Unique Binary String
Given an array of strings `nums` containing `n` **unique** binary strings each of length `n`, return *a binary string of length* `n` *that **does not appear** in* `nums`. *If there are multiple answers, you may return **any** of them*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = ["01","10"]
<strong>Output:</strong> "11"
<strong>Explanation:</strong> "11" does not appear in nums. "00" would also be correct.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = ["00","01"]
<strong>Output:</strong> "11"
<strong>Explanation:</strong> "11" does not appear in nums. "10" would also be correct.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = ["111","011","001"]
<strong>Output:</strong> "101"
<strong>Explanation:</strong> "101" does not appear in nums. "000", "010", "100", and "110" would also be correct.
</pre>

#### Constraints:
* `n == nums.length`
* `1 <= n <= 16`
* `nums[i].length == n`
* `nums[i]` is either `'0'` or `'1'`.
* All the strings of `nums` are **unique**.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def findDifferentBinaryString(self, nums: List[str]) -> str:
        nums = {int(num, 2) for num in nums}

        for x in range(2 ** len(nums)):
            if x not in nums:
                return bin(x)[2:].zfill(len(nums))
```
