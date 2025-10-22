# 1803. Count Pairs With XOR in a Range
Given a **(0-indexed)** integer array `nums` and two integers `low` and `high`, return *the number of **nice pairs***.

A **nice pair** is a pair `(i, j)` where `0 <= i < j < nums.length` and `low <= (nums[i] XOR nums[j]) <= high`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,4,2,7], low = 2, high = 6
<strong>Output:</strong> 6
<strong>Explanation:</strong> All nice pairs (i, j) are as follows:
    - (0, 1): nums[0] XOR nums[1] = 5
    - (0, 2): nums[0] XOR nums[2] = 3
    - (0, 3): nums[0] XOR nums[3] = 6
    - (1, 2): nums[1] XOR nums[2] = 6
    - (1, 3): nums[1] XOR nums[3] = 3
    - (2, 3): nums[2] XOR nums[3] = 5
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [9,8,4,2,1], low = 5, high = 14
<strong>Output:</strong> 8
<strong>Explanation:</strong> All nice pairs (i, j) are as follows:
    - (0, 2): nums[0] XOR nums[2] = 13
    - (0, 3): nums[0] XOR nums[3] = 11
    - (0, 4): nums[0] XOR nums[4] = 8
    - (1, 2): nums[1] XOR nums[2] = 12
    - (1, 3): nums[1] XOR nums[3] = 10
    - (1, 4): nums[1] XOR nums[4] = 9
    - (2, 3): nums[2] XOR nums[3] = 6
    - (2, 4): nums[2] XOR nums[4] = 5
</pre>

#### Constraints:
* <code>1 <= nums.length <= 2 * 10<sup>4</sup></code>
* <code>1 <= nums[i] <= 2 * 10<sup>4</sup></code>
* <code>1 <= low <= high <= 2 * 10<sup>4</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Trie:
    def __init__(self, maxheight: int, height: int):
        self.maxheight = maxheight
        self.height = height
        self.children = [None, None]
        self.count = 0

    def get(self, num: int, limit: int) -> int:
        if self.height == 0:
            return self.count

        ret = 0

        for i in range(2):
            if self.children[i]:
                num ^= (i << (self.height - 1))
                minxor = num & ~((1 << (self.height - 1)) - 1)
                maxxor = num | ((1 << (self.height - 1)) - 1)

                if maxxor <= limit:
                    ret += self.children[i].count
                elif minxor <= limit:
                    ret += self.children[i].get(num, limit)

        return ret

    def insert(self, num: int) -> None:
        self.count += 1

        if self.height > 0:
            i = (num >> (self.height - 1)) & 1
            if not self.children[i]:
                self.children[i] = Trie(self.maxheight, self.height - 1)
            self.children[i].insert(num)


class Solution:
    def countPairs(self, nums: List[int], low: int, high: int) -> int:
        maxheight = int(log2(max(high, max(nums)))) + 1
        trie = Trie(maxheight, maxheight)
        ret = 0

        for num in nums:
            ret += trie.get(num, high) - trie.get(num, low - 1)
            trie.insert(num)

        return ret
```
