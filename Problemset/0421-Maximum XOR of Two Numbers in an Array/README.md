# 421. Maximum XOR of Two Numbers in an Array
Given an integer array `nums`, return *the maximum result of* `nums[i] XOR nums[j]`, where `0 <= i <= j < n`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,10,5,25,2,8]
<strong>Output:</strong> 28
<strong>Explanation:</strong> The maximum result is 5 XOR 25 = 28.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [14,70,53,83,49,91,36,80,92,51,66,70]
<strong>Output:</strong> 127
</pre>

#### Constraints:
* <code>1 <= nums.length <= 2 * 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 2<sup>31</sup> - 1</code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def findMaximumXOR(self, nums: List[int]) -> int:
        trie = [[], []]
        ret = 0

        for num in nums:
            curr = trie
            for i in range(30, -1, -1):
                j = (num >> i) & 1
                if curr[j] == []:
                    curr[j].append([])
                    curr[j].append([])
                curr = curr[j]

            curr = trie
            x = 0
            for i in range(30, -1, -1):
                j = (num >> i) & 1
                if curr[j ^ 1] != []:
                    j ^= 1
                curr = curr[j]
                x |= j << i

            ret = max(ret, x ^ num)

        return ret
```
