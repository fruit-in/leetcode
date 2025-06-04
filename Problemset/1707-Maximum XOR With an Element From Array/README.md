# 1707. Maximum XOR With an Element From Array
You are given an array `nums` consisting of non-negative integers. You are also given a `queries` array, where <code>queries[i] = [x<sub>i</sub>, m<sub>i</sub>]</code>.

The answer to the <code>i<sup>th</sup></code> query is the maximum bitwise `XOR` value of <code>x<sub>i</sub></code> and any element of `nums` that does not exceed <code>m<sub>i</sub></code>. In other words, the answer is <code>max(nums[j] XOR x<sub>i</sub>)</code> for all `j` such that <code>nums[j] <= m<sub>i</sub></code>. If all elements in `nums` are larger than <code>m<sub>i</sub></code>, then the answer is `-1`.

Return *an integer array* `answer` *where* `answer.length == queries.length` *and* `answer[i]` *is the answer to the* <code>i<sup>th</sup></code> *query*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [0,1,2,3,4], queries = [[3,1],[1,3],[5,6]]
<strong>Output:</strong> [3,3,7]
<strong>Explanation:</strong>
1) 0 and 1 are the only two integers not greater than 1. 0 XOR 3 = 3 and 1 XOR 3 = 2. The larger of the two is 3.
2) 1 XOR 2 = 3.
3) 5 XOR 2 = 7.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [5,2,4,6,6,3], queries = [[12,4],[8,1],[6,3]]
<strong>Output:</strong> [15,-1,5]
</pre>

#### Constraints:
* <code>1 <= nums.length, queries.length <= 10<sup>5</sup></code>
* `queries[i].length == 2`
* <code>0 <= nums[j], x<sub>i</sub>, m<sub>i</sub> <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
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
