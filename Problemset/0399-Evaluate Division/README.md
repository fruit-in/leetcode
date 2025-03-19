# 399. Evaluate Division
You are given an array of variable pairs `equations` and an array of real numbers `values`, where <code>equations[i] = [A<sub>i</sub>, B<sub>i</sub>]</code> and `values[i]` represent the equation <code>A<sub>i</sub> / B<sub>i</sub> = values[i]</code>. Each <code>A<sub>i</sub></code> or <code>B<sub>i</sub></code> is a string that represents a single variable.

You are also given some `queries`, where <code>queries[j] = [C<sub>j</sub>, D<sub>j</sub>]</code> represents the <code>j<sup>th</sup></code> query where you must find the answer for <code>C<sub>j</sub> / D<sub>j</sub> = ?</code>.

Return *the answers to all queries*. If a single answer cannot be determined, return `-1.0`.

**Note:** The input is always valid. You may assume that evaluating the queries will not result in division by zero and that there is no contradiction.

**Note:** The variables that do not occur in the list of equations are undefined, so the answer cannot be determined for them.

#### Example 1:
<pre>
<strong>Input:</strong> equations = [["a","b"],["b","c"]], values = [2.0,3.0], queries = [["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]
<strong>Output:</strong> [6.00000,0.50000,-1.00000,1.00000,-1.00000]
<strong>Explanation:</strong>
Given: a / b = 2.0, b / c = 3.0
queries are: a / c = ?, b / a = ?, a / e = ?, a / a = ?, x / x = ?
return: [6.0, 0.5, -1.0, 1.0, -1.0 ]
note: x is undefined => -1.0
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> equations = [["a","b"],["b","c"],["bc","cd"]], values = [1.5,2.5,5.0], queries = [["a","c"],["c","b"],["bc","cd"],["cd","bc"]]
<strong>Output:</strong> [3.75000,0.40000,5.00000,0.20000]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> equations = [["a","b"]], values = [0.5], queries = [["a","b"],["b","a"],["a","c"],["x","y"]]
<strong>Output:</strong> [0.50000,2.00000,-1.00000,-1.00000]
</pre>

#### Constraints:
* `1 <= equations.length <= 20`
* `equations[i].length == 2`
* <code>1 <= A<sub>i</sub>.length, B<sub>i</sub>.length <= 5</code>
* `values.length == equations.length`
* `0.0 < values[i] <= 20.0`
* `1 <= queries.length <= 20`
* `queries[i].length == 2`
* <code>1 <= C<sub>j</sub>.length, D<sub>j</sub>.length <= 5</code>
* <code>A<sub>i</sub>, B<sub>i</sub>, C<sub>j</sub>, D<sub>j</sub></code> consist of lower case English letters and digits.

## Solutions (Python)

### 1. Solution
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
