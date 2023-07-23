# 77. 组合
给定两个整数 *n* 和 *k*，返回 1 ... *n* 中所有可能的 *k* 个数的组合。

#### 示例:
<pre>
<strong>输入:</strong> n = 4, k = 2
<strong>输出:</strong> [
  [2,4],
  [3,4],
  [2,3],
  [1,2],
  [1,3],
  [1,4],
]
</pre>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def combine(self, n: int, k: int) -> List[List[int]]:
        def foo(s: int, n: int, k: int) -> List[List[int]]:
            if k == 1:
                return [[i] for i in range(s, n + 1)]

            ret = []

            for i in range(s, n - k + 2):
                combs = foo(i + 1, n, k - 1)
                for comb in combs:
                    comb.append(i)
                ret.extend(combs)

            return ret

        return foo(1, n, k)
```
