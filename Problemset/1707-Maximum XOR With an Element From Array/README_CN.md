# 1707. 与数组中元素的最大异或值
给你一个由非负整数组成的数组 `nums` 。另有一个查询数组 `queries` ，其中 <code>queries[i] = [x<sub>i</sub>, m<sub>i</sub>]</code> 。

第 `i` 个查询的答案是 <code>x<sub>i</sub></code> 和任何 `nums` 数组中不超过 <code>m<sub>i</sub></code> 的元素按位异或（`XOR`）得到的最大值。换句话说，答案是 <code>max(nums[j] XOR x<sub>i</sub>)</code> ，其中所有 `j` 均满足 <code>nums[j] <= m<sub>i</sub></code> 。如果 `nums` 中的所有元素都大于 <code>m<sub>i</sub></code>，最终答案就是 `-1` 。

返回一个整数数组 `answer` 作为查询的答案，其中 `answer.length == queries.length` 且 `answer[i]` 是第 `i` 个查询的答案。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [0,1,2,3,4], queries = [[3,1],[1,3],[5,6]]
<strong>输出:</strong> [3,3,7]
<strong>解释:</strong>
1) 0 和 1 是仅有的两个不超过 1 的整数。0 XOR 3 = 3 而 1 XOR 3 = 2 。二者中的更大值是 3 。
2) 1 XOR 2 = 3.
3) 5 XOR 2 = 7.
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [5,2,4,6,6,3], queries = [[12,4],[8,1],[6,3]]
<strong>输出:</strong> [15,-1,5]
</pre>

#### 提示:
* <code>1 <= nums.length, queries.length <= 10<sup>5</sup></code>
* `queries[i].length == 2`
* <code>0 <= nums[j], x<sub>i</sub>, m<sub>i</sub> <= 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class TreeNode:
    def __init__(self, maxdepth=0, left=None, right=None, val=None):
        self.maxdepth = maxdepth
        self.left = left
        self.right = right
        self.val = val

    def insert(self, x, depth=0):
        if depth == self.maxdepth:
            self.val = x
            return

        bit = (x >> (self.maxdepth - depth - 1)) & 1

        if bit == 0:
            if not self.left:
                self.left = TreeNode(self.maxdepth)
            self.left.insert(x, depth + 1)
        else:
            if not self.right:
                self.right = TreeNode(self.maxdepth)
            self.right.insert(x, depth + 1)

    def getMaxXOR(self, x, depth=0):
        if depth == self.maxdepth:
            return self.val ^ x

        bit = (x >> (self.maxdepth - depth - 1)) & 1

        if (bit == 0 and not self.right) or (bit == 1 and self.left):
            return self.left.getMaxXOR(x, depth + 1)
        else:
            return self.right.getMaxXOR(x, depth + 1)


class Solution:
    def maximizeXor(self, nums: List[int], queries: List[List[int]]) -> List[int]:
        indices = sorted(range(len(queries)), key=lambda i: queries[i][1])
        root = TreeNode(ceil(log2(max(max(nums), max(x for x, _ in queries)))))
        answer = [-1] * len(queries)
        nums.sort(reverse=True)

        for i in indices:
            while nums and nums[-1] <= queries[i][1]:
                root.insert(nums.pop())

            if root.left or root.right:
                answer[i] = root.getMaxXOR(queries[i][0])

        return answer
```
