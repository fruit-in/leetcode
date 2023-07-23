# 2244. Minimum Rounds to Complete All Tasks
You are given a **0-indexed** integer array `tasks`, where `tasks[i]` represents the difficulty level of a task. In each round, you can complete either 2 or 3 tasks of the **same difficulty level**.

Return *the **minimum** rounds required to complete all the tasks, or* `-1` *if it is not possible to complete all the tasks*.

#### Example 1:
<pre>
<strong>Input:</strong> tasks = [2,2,3,3,2,4,4,4,4,4]
<strong>Output:</strong> 4
<strong>Explanation:</strong> To complete all the tasks, a possible plan is:
- In the first round, you complete 3 tasks of difficulty level 2.
- In the second round, you complete 2 tasks of difficulty level 3.
- In the third round, you complete 3 tasks of difficulty level 4.
- In the fourth round, you complete 2 tasks of difficulty level 4.
It can be shown that all the tasks cannot be completed in fewer than 4 rounds, so the answer is 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> tasks = [2,3,3]
<strong>Output:</strong> -1
<strong>Explanation:</strong> There is only 1 task of difficulty level 2, but in each round, you can only complete either 2 or 3 tasks of the same difficulty level. Hence, you cannot complete all the tasks, and the answer is -1.
</pre>

#### Constraints:
* <code>1 <= tasks.length <= 10<sup>5</sup></code>
* <code>1 <= tasks[i] <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def minimumRounds(self, tasks: List[int]) -> int:
        count = Counter(tasks)
        ret = 0

        for v in count.values():
            if v == 1:
                return -1

            ret += v // 3 + (1 if v % 3 > 0 else 0)

        return ret
```
