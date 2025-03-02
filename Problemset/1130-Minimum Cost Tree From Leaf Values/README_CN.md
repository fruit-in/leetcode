# 1130. 叶值的最小代价生成树
给你一个正整数数组 `arr`，考虑所有满足以下条件的二叉树：
* 每个节点都有 `0` 个或是 `2` 个子节点。
* 数组 `arr` 中的值与树的中序遍历中每个叶节点的值一一对应。
* 每个非叶节点的值等于其左子树和右子树中叶节点的最大值的乘积。

在所有这样的二叉树中，返回每个非叶节点的值的最小可能总和。这个和的值是一个 32 位整数。

如果一个节点有 0 个子节点，那么该节点为叶节点。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/08/10/tree1.jpg)
<pre>
<strong>输入:</strong> arr = [6,2,4]
<strong>输出:</strong> 32
<strong>解释:</strong> 有两种可能的树，第一种的非叶节点的总和为 36 ，第二种非叶节点的总和为 32 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/08/10/tree2.jpg)
<pre>
<strong>输入:</strong> arr = [4,11]
<strong>输出:</strong> 44
</pre>

#### 提示:
* `2 <= arr.length <= 40`
* `1 <= arr[i] <= 15`
* 答案保证是一个 32 位带符号整数，即小于 <code>2<sup>31</sup></code> 。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def mctFromLeafValues(self, arr: List[int]) -> int:
        @cache
        def mctFromSub(i: int, j: int) -> (int, int):
            if i + 1 == j:
                return (0, arr[i])

            treesum, treemax = 1 << 31, 0

            for k in range(i + 1, j):
                leftsum, leftmax = mctFromSub(i, k)
                rightsum, rightmax = mctFromSub(k, j)
                treesum, treemax = min(
                    treesum, leftsum + rightsum + leftmax * rightmax), max(leftmax, rightmax)

            return (treesum, treemax)

        return mctFromSub(0, len(arr))[0]
```
