# 1494. 并行课程 II
给你一个整数 `n` 表示某所大学里课程的数目，编号为 `1` 到 `n` ，数组 `relations` 中， <code>relations[i] = [x<sub>i</sub>, y<sub>i</sub>]</code>  表示一个先修课的关系，也就是课程 <code>x<sub>i</sub></code> 必须在课程 <code>y<sub>i</sub></code> 之前上。同时你还有一个整数 `k` 。

在一个学期中，你 **最多** 可以同时上 `k` 门课，前提是这些课的先修课在之前的学期里已经上过了。

请你返回上完所有课最少需要多少个学期。题目保证一定存在一种上完所有课的方式。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/05/22/leetcode_parallel_courses_1.png)
<pre>
<strong>输入:</strong> n = 4, relations = [[2,1],[3,1],[1,4]], k = 2
<strong>输出:</strong> 3
<strong>解释:</strong> 上图展示了题目输入的图。在第一个学期中，我们可以上课程 2 和课程 3 。然后第二个学期上课程 1 ，第三个学期上课程 4 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/05/22/leetcode_parallel_courses_2.png)
<pre>
<strong>输入:</strong> n = 5, relations = [[2,1],[3,1],[4,1],[1,5]], k = 2
<strong>输出:</strong> 4
<strong>解释:</strong> 上图展示了题目输入的图。一个最优方案是：第一学期上课程 2 和 3，第二学期上课程 4 ，第三学期上课程 1 ，第四学期上课程 5 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 11, relations = [], k = 2
<strong>输出:</strong> 6
</pre>

#### 提示:
* `1 <= n <= 15`
* `1 <= k <= n`
* `0 <= relations.length <= n * (n-1) / 2`
* `relations[i].length == 2`
* <code>1 <= x<sub>i</sub>, y<sub>i</sub> <= n</code>
* <code>x<sub>i</sub> != y<sub>i</sub></code>
* 所有先修关系都是不同的，也就是说 `relations[i] != relations[j]` 。
* 题目输入的图是个有向无环图。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def minNumberOfSemesters(self, n: int, relations: List[List[int]], k: int) -> int:
        @cache
        def dfs(mask: int) -> int:
            if mask == (1 << n) - 1:
                return 0

            candidates = [x for x in range(n) if (
                mask >> x) & 1 == 0 and mask & prevcourses[x] == prevcourses[x]]
            ret = n

            for comb in combinations(candidates, min(k, len(candidates))):
                newmask = reduce(lambda x, y: x | (1 << y), comb, mask)
                ret = min(ret, 1 + dfs(newmask))

            return ret

        prevcourses = [0 for _ in range(n)]

        for x, y in relations:
            prevcourses[y - 1] |= 1 << (x - 1)

        return dfs(0)
```
