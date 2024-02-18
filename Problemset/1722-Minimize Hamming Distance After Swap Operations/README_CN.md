# 1722. 执行交换操作后的最小汉明距离
给你两个整数数组 `source` 和 `target` ，长度都是 `n` 。还有一个数组 `allowedSwaps` ，其中每个 <code>allowedSwaps[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> 表示你可以交换数组 `source` 中下标为 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code>（**下标从 0 开始**）的两个元素。注意，你可以按 **任意** 顺序 **多次** 交换一对特定下标指向的元素。

相同长度的两个数组 `source` 和 `target` 间的 **汉明距离** 是元素不同的下标数量。形式上，其值等于满足 `source[i] != target[i]` （**下标从 0 开始**）的下标 `i`（`0 <= i <= n-1`）的数量。

在对数组 `source` 执行 **任意** 数量的交换操作后，返回 `source` 和 `target` 间的 **最小汉明距离** 。

#### 示例 1:
<pre>
<strong>输入:</strong> source = [1,2,3,4], target = [2,1,4,5], allowedSwaps = [[0,1],[2,3]]
<strong>输出:</strong> 1
<strong>解释:</strong> source 可以按下述方式转换：
- 交换下标 0 和 1 指向的元素：source = [2,1,3,4]
- 交换下标 2 和 3 指向的元素：source = [2,1,4,3]
source 和 target 间的汉明距离是 1 ，二者有 1 处元素不同，在下标 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> source = [1,2,3,4], target = [1,3,2,4], allowedSwaps = []
<strong>输出:</strong> 2
<strong>解释:</strong> 不能对 source 执行交换操作。
source 和 target 间的汉明距离是 2 ，二者有 2 处元素不同，在下标 1 和下标 2 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> source = [5,1,2,4,3], target = [1,5,4,2,3], allowedSwaps = [[0,4],[4,2],[1,3],[1,4]]
<strong>输出:</strong> 0
</pre>

#### 提示:
* `n == source.length == target.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= source[i], target[i] <= 10<sup>5</sup></code>
* <code>0 <= allowedSwaps.length <= 10<sup>5</sup></code>
* `allowedSwaps[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> <= n - 1</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def minimumHammingDistance(self, source: List[int], target: List[int], allowedSwaps: List[List[int]]) -> int:
        n = len(source)
        parent = list(range(n))
        groups = {}
        ret = n

        for a, b in allowedSwaps:
            while parent[a] != parent[parent[a]]:
                parent[a] = parent[parent[a]]
            while parent[b] != parent[parent[b]]:
                parent[b] = parent[parent[b]]

            if parent[a] < parent[b]:
                parent[parent[b]] = parent[a]
            else:
                parent[parent[a]] = parent[b]

        for i in range(n):
            while parent[i] != parent[parent[i]]:
                parent[i] = parent[parent[i]]

            if parent[i] not in groups:
                groups[parent[i]] = {}
            if source[i] not in groups[parent[i]]:
                groups[parent[i]][source[i]] = 0
            groups[parent[i]][source[i]] += 1

        for i in range(n):
            if groups[parent[i]].get(target[i], 0) > 0:
                groups[parent[i]][target[i]] -= 1
                ret -= 1

        return ret
```
