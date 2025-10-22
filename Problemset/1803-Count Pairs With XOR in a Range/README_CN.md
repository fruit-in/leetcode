# 1803. 统计异或值在范围内的数对有多少
给你一个整数数组 `nums` （下标 **从 0 开始** 计数）以及两个整数：`low` 和 `high` ，请返回 **漂亮数对** 的数目。

**漂亮数对** 是一个形如 `(i, j)` 的数对，其中 `0 <= i < j < nums.length` 且 `low <= (nums[i] XOR nums[j]) <= high` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,4,2,7], low = 2, high = 6
<strong>输出:</strong> 6
<strong>解释:</strong> 所有漂亮数对 (i, j) 列出如下：
    - (0, 1): nums[0] XOR nums[1] = 5
    - (0, 2): nums[0] XOR nums[2] = 3
    - (0, 3): nums[0] XOR nums[3] = 6
    - (1, 2): nums[1] XOR nums[2] = 6
    - (1, 3): nums[1] XOR nums[3] = 3
    - (2, 3): nums[2] XOR nums[3] = 5
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [9,8,4,2,1], low = 5, high = 14
<strong>输出:</strong> 8
<strong>解释:</strong> 所有漂亮数对 (i, j) 列出如下：
    - (0, 2): nums[0] XOR nums[2] = 13
    - (0, 3): nums[0] XOR nums[3] = 11
    - (0, 4): nums[0] XOR nums[4] = 8
    - (1, 2): nums[1] XOR nums[2] = 12
    - (1, 3): nums[1] XOR nums[3] = 10
    - (1, 4): nums[1] XOR nums[4] = 9
    - (2, 3): nums[2] XOR nums[3] = 6
    - (2, 4): nums[2] XOR nums[4] = 5
</pre>

#### 提示:
* <code>1 <= nums.length <= 2 * 10<sup>4</sup></code>
* <code>1 <= nums[i] <= 2 * 10<sup>4</sup></code>
* <code>1 <= low <= high <= 2 * 10<sup>4</sup></code>

## 题解 (Python)

### 1. 题解
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
