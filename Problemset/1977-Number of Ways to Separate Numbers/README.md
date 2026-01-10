# 1494. Parallel Courses II
You are given an integer `n`, which indicates that there are `n` courses labeled from `1` to `n`. You are also given an array `relations` where <code>relations[i] = [prevCourse<sub>i</sub>, nextCourse<sub>i</sub>]</code>, representing a prerequisite relationship between course <code>prevCourse<sub>i</sub></code> and course <code>nextCourse<sub>i</sub></code>: course <code>prevCourse<sub>i</sub></code> has to be taken before course <code>nextCourse<sub>i</sub></code>. Also, you are given the integer `k`.

In one semester, you can take **at most** `k` courses as long as you have taken all the prerequisites in the **previous** semesters for the courses you are taking.

Return *the **minimum** number of semesters needed to take all courses*. The testcases will be generated such that it is possible to take every course.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/05/22/leetcode_parallel_courses_1.png)
<pre>
<strong>Input:</strong> n = 4, relations = [[2,1],[3,1],[1,4]], k = 2
<strong>Output:</strong> 3
<strong>Explanation:</strong> The figure above represents the given graph.
In the first semester, you can take courses 2 and 3.
In the second semester, you can take course 1.
In the third semester, you can take course 4.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/05/22/leetcode_parallel_courses_2.png)
<pre>
<strong>Input:</strong> n = 5, relations = [[2,1],[3,1],[4,1],[1,5]], k = 2
<strong>Output:</strong> 4
<strong>Explanation:</strong> The figure above represents the given graph.
In the first semester, you can only take courses 2 and 3 since you cannot take more than two per semester.
In the second semester, you can take course 4.
In the third semester, you can take course 1.
In the fourth semester, you can take course 5.
</pre>

#### Constraints:
* `1 <= n <= 15`
* `1 <= k <= n`
* `0 <= relations.length <= n * (n-1) / 2`
* `relations[i].length == 2`
* <code>1 <= prevCourse<sub>i</sub>, nextCourse<sub>i</sub> <= n</code>
* <code>prevCourse<sub>i</sub> != nextCourse<sub>i</sub></code>
* All the pairs <code>[prevCourse<sub>i</sub>, nextCourse<sub>i</sub>]</code> are unique.
* The given graph is a directed acyclic graph.

## Solutions (Python)

### 1. Solution
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
