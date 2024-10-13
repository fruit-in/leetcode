# 2476. 二叉搜索树最近节点查询
给你一个 **二叉搜索树** 的根节点 `root` ，和一个由正整数组成、长度为 `n` 的数组 `queries` 。

请你找出一个长度为 `n` 的 **二维** 答案数组 `answer` ，其中 <code>answer[i] = [min<sub>i</sub>, max<sub>i</sub>]</code> ：

* <code>min<sub>i</sub></code> 是树中小于等于 `queries[i]` 的 **最大值** 。如果不存在这样的值，则使用 `-1` 代替。
* <code>max<sub>i</sub></code> 是树中大于等于 `queries[i]` 的 **最小值** 。如果不存在这样的值，则使用 `-1` 代替。

返回数组 `answer` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/09/28/bstreeedrawioo.png)
<pre>
<strong>输入:</strong> root = [6,2,13,1,4,9,15,null,null,null,null,null,null,14], queries = [2,5,16]
<strong>输出:</strong> [[2,2],[4,6],[15,-1]]
<strong>解释:</strong> 按下面的描述找出并返回查询的答案：
- 树中小于等于 2 的最大值是 2 ，且大于等于 2 的最小值也是 2 。所以第一个查询的答案是 [2,2] 。
- 树中小于等于 5 的最大值是 4 ，且大于等于 5 的最小值是 6 。所以第二个查询的答案是 [4,6] 。
- 树中小于等于 16 的最大值是 15 ，且大于等于 16 的最小值不存在。所以第三个查询的答案是 [15,-1] 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/09/28/bstttreee.png)
<pre>
<strong>输入:</strong> root = [4,null,9], queries = [3]
<strong>输出:</strong> [[-1,4]]
<strong>解释:</strong> 树中不存在小于等于 3 的最大值，且大于等于 3 的最小值是 4 。所以查询的答案是 [-1,4] 。
</pre>

#### 提示:
* 树中节点的数目在范围 <code>[2, 10<sup>5</sup>]</code> 内
* <code>1 <= Node.val <= 10<sup>6</sup></code>
* `n == queries.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= queries[i] <= 10<sup>6</sup></code>

## 题解 (Python)

### 1. 题解
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def closestNodes(self, root: Optional[TreeNode], queries: List[int]) -> List[List[int]]:
        vals = []
        answer = []

        def dfs(root):
            if root is not None:
                dfs(root.left)
                vals.append(root.val)
                dfs(root.right)

        dfs(root)

        for x in queries:
            i = bisect.bisect(vals, x)
            j = bisect.bisect_left(vals, x)
            answer.append([vals[i - 1] if i > 0 else -1,
                          vals[j] if j < len(vals) else -1])

        return answer
```
