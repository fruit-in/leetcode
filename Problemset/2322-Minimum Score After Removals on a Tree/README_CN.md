# 2322. 从树中删除边的最小分数
存在一棵无向连通树，树中有编号从 `0` 到 `n - 1` 的 `n` 个节点， 以及 `n - 1` 条边。

给你一个下标从 **0** 开始的整数数组 `nums` ，长度为 `n` ，其中 `nums[i]` 表示第 `i` 个节点的值。另给你一个二维整数数组 `edges` ，长度为 `n - 1` ，其中 <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> 表示树中存在一条位于节点 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code> 之间的边。

删除树中两条 **不同** 的边以形成三个连通组件。对于一种删除边方案，定义如下步骤以计算其分数：
1. 分别获取三个组件 **每个** 组件中所有节点值的异或值。
2. **最大** 异或值和 **最小** 异或值的 **差值** 就是这一种删除边方案的分数。

* 例如，三个组件的节点值分别是：`[4,5,7]`、`[1,9]` 和 `[3,3,3]` 。三个异或值分别是 `4 ^ 5 ^ 7 = 6`、`1 ^ 9 = 8` 和 `3 ^ 3 ^ 3 = 3` 。最大异或值是 `8` ，最小异或值是 `3` ，分数是 `8 - 3 = 5` 。

返回在给定树上执行任意删除边方案可能的 **最小** 分数。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/05/03/ex1drawio.png)
<pre>
<strong>输入:</strong> nums = [1,5,5,4,11], edges = [[0,1],[1,2],[1,3],[3,4]]
<strong>输出:</strong> 9
<strong>解释:</strong> 上图展示了一种删除边方案。
- 第 1 个组件的节点是 [1,3,4] ，值是 [5,4,11] 。异或值是 5 ^ 4 ^ 11 = 10 。
- 第 2 个组件的节点是 [0] ，值是 [1] 。异或值是 1 = 1 。
- 第 3 个组件的节点是 [2] ，值是 [5] 。异或值是 5 = 5 。
分数是最大异或值和最小异或值的差值，10 - 1 = 9 。
可以证明不存在分数比 9 小的删除边方案。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/05/03/ex2drawio.png)
<pre>
<strong>输入:</strong> nums = [5,5,2,4,4,2], edges = [[0,1],[1,2],[5,2],[4,3],[1,3]]
<strong>输出:</strong> 0
<strong>解释:</strong> 上图展示了一种删除边方案。
- 第 1 个组件的节点是 [3,4] ，值是 [4,4] 。异或值是 4 ^ 4 = 0 。
- 第 2 个组件的节点是 [1,0] ，值是 [5,5] 。异或值是 5 ^ 5 = 0 。
- 第 3 个组件的节点是 [2,5] ，值是 [2,2] 。异或值是 2 ^ 2 = 0 。
分数是最大异或值和最小异或值的差值，0 - 0 = 0 。
无法获得比 0 更小的分数 0 。
</pre>

#### 提示:
* `n == nums.length`
* `3 <= n <= 1000`
* <code>1 <= nums[i] <= 10<sup>8</sup></code>
* `edges.length == n - 1`
* `edges[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> < n</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* `edges` 表示一棵有效的树

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def minimumScore(self, nums: List[int], edges: List[List[int]]) -> int:
        def dfs(i: int) -> None:
            for j in children[i]:
                children[j].remove(i)
                ancestors[j] = ancestors[i] | {i}
                dfs(j)
                subtreexor[i] ^= subtreexor[j]

        n = len(nums)
        children = [set() for _ in range(n)]
        ancestors = [set() for _ in range(n)]
        subtreexor = nums.copy()
        ret = inf

        for a, b in edges:
            children[a].add(b)
            children[b].add(a)

        dfs(0)

        for edge in edges:
            if edge[0] in children[edge[1]]:
                edge[0], edge[1] = edge[1], edge[0]

        for i in range(n - 1):
            a, b = edges[i]
            for j in range(i + 1, n - 1):
                c, d = edges[j]
                xor1 = subtreexor[b]
                xor2 = subtreexor[d]
                if c == b or b in ancestors[c]:
                    xor1 ^= xor2
                elif a == d or d in ancestors[a]:
                    xor2 ^= xor1
                xor3 = subtreexor[0] ^ xor1 ^ xor2
                ret = min(ret, max(xor1, xor2, xor3) - min(xor1, xor2, xor3))

        return ret
```
