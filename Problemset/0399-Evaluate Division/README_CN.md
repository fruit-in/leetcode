# 399. 除法求值
给你一个变量对数组 `equations` 和一个实数值数组 `values` 作为已知条件，其中 <code>equations[i] = [A<sub>i</sub>, B<sub>i</sub>]</code> 和 `values[i]` 共同表示等式 <code>A<sub>i</sub> / B<sub>i</sub> = values[i]</code> 。每个 <code>A<sub>i</sub></code> 或 <code>B<sub>i</sub></code> 是一个表示单个变量的字符串。

另有一些以数组 `queries` 表示的问题，其中 <code>queries[j] = [C<sub>j</sub>, D<sub>j</sub>]</code> 表示第 `j` 个问题，请你根据已知条件找出 <code>C<sub>j</sub> / D<sub>j</sub> = ?</code> 的结果作为答案。

返回 **所有问题的答案** 。如果存在某个无法确定的答案，则用 `-1.0` 替代这个答案。如果问题中出现了给定的已知条件中没有出现的字符串，也需要用 `-1.0` 替代这个答案。

**注意：**输入总是有效的。你可以假设除法运算中不会出现除数为 0 的情况，且不存在任何矛盾的结果。

**注意：**未在等式列表中出现的变量是未定义的，因此无法确定它们的答案。

#### 示例 1:
<pre>
<strong>输入:</strong> equations = [["a","b"],["b","c"]], values = [2.0,3.0], queries = [["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]
<strong>输出:</strong> [6.00000,0.50000,-1.00000,1.00000,-1.00000]
<strong>解释:</strong>
条件：a / b = 2.0, b / c = 3.0
问题：a / c = ?, b / a = ?, a / e = ?, a / a = ?, x / x = ?
结果：[6.0, 0.5, -1.0, 1.0, -1.0 ]
注意：x 是未定义的 => -1.0
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> equations = [["a","b"],["b","c"],["bc","cd"]], values = [1.5,2.5,5.0], queries = [["a","c"],["c","b"],["bc","cd"],["cd","bc"]]
<strong>输出:</strong> [3.75000,0.40000,5.00000,0.20000]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> equations = [["a","b"]], values = [0.5], queries = [["a","b"],["b","a"],["a","c"],["x","y"]]
<strong>输出:</strong> [0.50000,2.00000,-1.00000,-1.00000]
</pre>

#### 提示:
* `1 <= equations.length <= 20`
* `equations[i].length == 2`
* <code>1 <= A<sub>i</sub>.length, B<sub>i</sub>.length <= 5</code>
* `values.length == equations.length`
* `0.0 < values[i] <= 20.0`
* `1 <= queries.length <= 20`
* `queries[i].length == 2`
* <code>1 <= C<sub>j</sub>.length, D<sub>j</sub>.length <= 5</code>
* <code>A<sub>i</sub>, B<sub>i</sub>, C<sub>j</sub>, D<sub>j</sub></code> 由小写英文字母与数字组成

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def calcEquation(self, equations: List[List[str]], values: List[float], queries: List[List[str]]) -> List[float]:
        parent = {}
        variablevalue = {}
        ret = []

        for (a, b), v in zip(equations, values):
            if a not in parent:
                variablevalue[a] = 1.
                parent[a] = a
            if b not in parent:
                variablevalue[b] = 1.
                parent[b] = b

            while parent[a] != parent[parent[a]]:
                variablevalue[a] *= variablevalue[parent[a]]
                parent[a] = parent[parent[a]]
            while parent[b] != parent[parent[b]]:
                variablevalue[b] *= variablevalue[parent[b]]
                parent[b] = parent[parent[b]]

            if parent[a] < parent[b]:
                variablevalue[parent[b]] = variablevalue[a] / \
                    variablevalue[b] / v
                parent[parent[b]] = parent[a]
            else:
                variablevalue[parent[a]] = variablevalue[b] / \
                    variablevalue[a] * v
                parent[parent[a]] = parent[b]

        for a in parent:
            while parent[a] != parent[parent[a]]:
                variablevalue[a] *= variablevalue[parent[a]]
                parent[a] = parent[parent[a]]

        for c, d in queries:
            if c in parent and d in parent and parent[c] == parent[d]:
                ret.append(variablevalue[c] / variablevalue[d])
            else:
                ret.append(-1.)

        return ret
```
