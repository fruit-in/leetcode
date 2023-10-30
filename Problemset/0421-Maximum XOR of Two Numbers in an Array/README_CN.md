# 421. 数组中两个数的最大异或值
给你一个整数数组 `nums` ，返回 `nums[i] XOR nums[j]` 的最大运算结果，其中 `0 ≤ i ≤ j < n` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,10,5,25,2,8]
<strong>输出:</strong> 28
<strong>解释:</strong> 最大运算结果是 5 XOR 25 = 28.
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [14,70,53,83,49,91,36,80,92,51,66,70]
<strong>输出:</strong> 127
</pre>

#### 提示:
* <code>1 <= nums.length <= 2 * 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 2<sup>31</sup> - 1</code>

## 题解 (Python)

### 1. 题解
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
