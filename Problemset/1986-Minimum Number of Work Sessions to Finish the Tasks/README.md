# 1986. Minimum Number of Work Sessions to Finish the Tasks
There are `n` tasks assigned to you. The task times are represented as an integer array `tasks` of length `n`, where the <code>i<sup>th</sup></code> task takes `tasks[i]` hours to finish. A **work session** is when you work for **at most** `sessionTime` consecutive hours and then take a break.

You should finish the given tasks in a way that satisfies the following conditions:
* If you start a task in a work session, you must complete it in the **same** work session.
* You can start a new task **immediately** after finishing the previous one.
* You may complete the tasks in **any order**.

Given `tasks` and `sessionTime`, return *the **minimum** number of **work sessions** needed to finish all the tasks following the conditions above*.

The tests are generated such that `sessionTime` is **greater** than or equal to the **maximum** element in `tasks[i]`.

#### Example 1:
<pre>
<strong>Input:</strong> tasks = [1,2,3], sessionTime = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> You can finish the tasks in two work sessions.
- First work session: finish the first and the second tasks in 1 + 2 = 3 hours.
- Second work session: finish the third task in 3 hours.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> tasks = [3,1,3,1,1], sessionTime = 8
<strong>Output:</strong> 2
<strong>Explanation:</strong> You can finish the tasks in two work sessions.
- First work session: finish all the tasks except the last one in 3 + 1 + 3 + 1 = 8 hours.
- Second work session: finish the last task in 1 hour.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> tasks = [1,2,3,4,5], sessionTime = 15
<strong>Output:</strong> 1
<strong>Explanation:</strong> You can finish all the tasks in one work session.
</pre>

#### Constraints:
* `n == tasks.length`
* `1 <= n <= 14`
* `1 <= tasks[i] <= 10`
* `max(tasks[i]) <= sessionTime <= 15`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def minSessions(self, tasks: List[int], sessionTime: int) -> int:
        @cache
        def dfs(mask: int, remain: int) -> int:
            if mask == 0:
                return 0

            ret = inf

            for i in range(len(tasks)):
                if (mask >> i) & 1 == 1:
                    if tasks[i] <= remain:
                        ret = min(ret, dfs(mask ^ (1 << i), remain - tasks[i]))
                    else:
                        ret = min(ret, 1 + dfs(mask ^ (1 << i),
                                  sessionTime - tasks[i]))

            return ret

        return dfs((1 << len(tasks)) - 1, 0)
```
