# 996. 平方数组的数目
如果一个数组的任意两个相邻元素之和都是 **完全平方数** ，则该数组称为 **平方数组** 。

给定一个整数数组 `nums`，返回所有属于 **平方数组** 的 `nums` 的排列数量。

如果存在某个索引 `i` 使得 `perm1[i] != perm2[i]`，则认为两个排列 `perm1` 和 `perm2` 不同。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,17,8]
<strong>输出:</strong> 2
<strong>解释:</strong> [1,8,17] 和 [17,8,1] 是有效的排列。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,2,2]
<strong>输出:</strong> 1
</pre>

#### 提示:
* `1 <= nums.length <= 12`
* <code>0 <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def numSquarefulPerms(self, nums: List[int]) -> int:
        n = len(nums)
        neighbors = {x: set() for x in nums} | {None: set(nums)}
        remain = {x: 0 for x in nums}
        ret = 0

        for i in range(n):
            x = nums[i]
            remain[x] += 1
            for j in range(i + 1, n):
                y = nums[j]
                if int(math.sqrt(x + y)) ** 2 == x + y:
                    neighbors[x].add(y)
                    neighbors[y].add(x)

        def dfs(i: int, prev: Optional[int]) -> None:
            nonlocal ret
            if i == n:
                ret += 1
                return

            for x in neighbors[prev]:
                if remain[x] > 0:
                    remain[x] -= 1
                    dfs(i + 1, x)
                    remain[x] += 1

        dfs(0, None)

        return ret
```
