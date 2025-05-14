# 1723. Find Minimum Time to Finish All Jobs
You are given an integer array `jobs`, where `jobs[i]` is the amount of time it takes to complete the <code>i<sup>th</sup></code> job.

There are `k` workers that you can assign jobs to. Each job should be assigned to **exactly** one worker. The **working time** of a worker is the sum of the time it takes to complete all jobs assigned to them. Your goal is to devise an optimal assignment such that the **maximum working time** of any worker is **minimized**.

*Return the **minimum** possible **maximum working time** of any assignment*.

#### Example 1:
<pre>
<strong>Input:</strong> jobs = [3,2,3], k = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> By assigning each person one job, the maximum time is 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> jobs = [1,2,4,7,8], k = 2
<strong>Output:</strong> 11
<strong>Explanation:</strong> Assign the jobs the following way:
Worker 1: 1, 2, 8 (working time = 1 + 2 + 8 = 11)
Worker 2: 4, 7 (working time = 4 + 7 = 11)
The maximum working time is 11.
</pre>

#### Constraints:
* `1 <= k <= jobs.length <= 12`
* <code>1 <= jobs[i] <= 10<sup>7</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def minimumTimeRequired(self, jobs: List[int], k: int) -> int:
        def dfs(i: int) -> None:
            nonlocal ret
            if i == len(jobs):
                ret = min(ret, max(workers))
                return

            for j in range(k):
                if j > 0 and workers[j - 1] == 0:
                    break
                if workers[j] + jobs[i] >= ret:
                    continue
                workers[j] += jobs[i]
                dfs(i + 1)
                workers[j] -= jobs[i]

        workers = [0] * k
        ret = sum(jobs)

        dfs(0)

        return ret
```
