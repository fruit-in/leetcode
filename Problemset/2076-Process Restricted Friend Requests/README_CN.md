# 2076. 处理含限制条件的好友请求
给你一个整数 `n` ，表示网络上的用户数目。每个用户按从 `0` 到 `n - 1` 进行编号。

给你一个下标从 **0** 开始的二维整数数组 `restrictions` ，其中 <code>restrictions[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> 意味着用户 <code>x<sub>i</sub></code> 和用户 <code>y<sub>i</sub></code> **不能** 成为 **朋友** ，不管是 **直接** 还是通过其他用户 **间接** 。

最初，用户里没有人是其他用户的朋友。给你一个下标从 **0** 开始的二维整数数组 `requests` 表示好友请求的列表，其中 <code>requests[j] = [u<sub>j</sub>, v<sub>j</sub>]</code> 是用户 <code>u<sub>j</sub></code> 和用户 <code>v<sub>j</sub></code> 之间的一条好友请求。

如果 <code>u<sub>j</sub></code> 和 <code>v<sub>j</sub></code> 可以成为 **朋友** ，那么好友请求将会 **成功** 。每个好友请求都会按列表中给出的顺序进行处理（即，`requests[j]` 会在 `requests[j + 1]` 前）。一旦请求成功，那么对所有未来的好友请求而言， <code>u<sub>j</sub></code> 和 <code>v<sub>j</sub></code> 将会 **成为直接朋友** 。

返回一个 **布尔数组** `result` ，其中元素遵循此规则：如果第 `j` 个好友请求 **成功** ，那么 `result[j]` 就是 `true` ；否则，为 `false` 。

**注意：**如果 <code>u<sub>j</sub></code> 和 <code>v<sub>j</sub></code> 已经是直接朋友，那么他们之间的请求将仍然 **成功** 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 3, restrictions = [[0,1]], requests = [[0,2],[2,1]]
<strong>输出:</strong> [true,false]
<strong>解释:</strong>
请求 0 ：用户 0 和 用户 2 可以成为朋友，所以他们成为直接朋友。
请求 1 ：用户 2 和 用户 1 不能成为朋友，因为这会使 用户 0 和 用户 1 成为间接朋友 (1--2--0) 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 3, restrictions = [[0,1]], requests = [[1,2],[0,2]]
<strong>输出:</strong> [true,false]
<strong>解释:</strong>
请求 0 ：用户 1 和 用户 2 可以成为朋友，所以他们成为直接朋友。
请求 1 ：用户 0 和 用户 2 不能成为朋友，因为这会使 用户 0 和 用户 1 成为间接朋友 (0--2--1) 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 5, restrictions = [[0,1],[1,2],[2,3]], requests = [[0,4],[1,2],[3,1],[3,4]]
<strong>输出:</strong> [true,false,true,false]
<strong>解释:</strong>
请求 0 ：用户 0 和 用户 4 可以成为朋友，所以他们成为直接朋友。
请求 1 ：用户 1 和 用户 2 不能成为朋友，因为他们之间存在限制。
请求 2 ：用户 3 和 用户 1 可以成为朋友，所以他们成为直接朋友。
请求 3 ：用户 3 和 用户 4 不能成为朋友，因为这会使 用户 0 和 用户 1 成为间接朋友 (0--4--3--1) 。
</pre>

#### 提示:
* `2 <= n <= 1000`
* `0 <= restrictions.length <= 1000`
* `restrictions[i].length == 2`
* <code>0 <= x<sub>i</sub>, y<sub>i</sub> <= n - 1</code>
* <code>x<sub>i</sub> != y<sub>i</sub></code>
* `1 <= requests.length <= 1000`
* `requests[j].length == 2`
* <code>0 <= u<sub>j</sub>, v<sub>j</sub> <= n - 1</code>
* <code>u<sub>j</sub> != v<sub>j</sub></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def friendRequests(self, n: int, restrictions: List[List[int]], requests: List[List[int]]) -> List[bool]:
        parent = {p: p for p in range(n)}
        group = {p: {p} for p in range(n)}
        result = [True] * len(requests)

        for i, [u, v] in enumerate(requests):
            for x, y in restrictions:
                if (x in group[parent[u]] and y in group[parent[v]]) or (y in group[parent[u]] and x in group[parent[v]]):
                    result[i] = False
                    break

            if result[i] and parent[u] != parent[v]:
                if len(group[parent[u]]) < len(group[parent[v]]):
                    u, v = v, u
                for p in group.pop(parent[v]):
                    parent[p] = parent[u]
                    group[parent[u]].add(p)

        return result
```
