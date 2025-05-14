# 1723. 完成所有工作的最短时间
给你一个整数数组 `jobs` ，其中 `jobs[i]` 是完成第 `i` 项工作要花费的时间。

请你将这些工作分配给 `k` 位工人。所有工作都应该分配给工人，且每项工作只能分配给一位工人。工人的 **工作时间** 是完成分配给他们的所有工作花费时间的总和。请你设计一套最佳的工作分配方案，使工人的 **最大工作时间** 得以 **最小化** 。

返回分配方案中尽可能 **最小** 的 **最大工作时间** 。

#### 示例 1:
<pre>
<strong>输入:</strong> jobs = [3,2,3], k = 3
<strong>输出:</strong> 3
<strong>解释:</strong> 给每位工人分配一项工作，最大工作时间是 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> jobs = [1,2,4,7,8], k = 2
<strong>输出:</strong> 11
<strong>解释:</strong> 按下述方式分配工作：
1 号工人：1、2、8（工作时间 = 1 + 2 + 8 = 11）
2 号工人：4、7（工作时间 = 4 + 7 = 11）
最大工作时间是 11 。
</pre>

#### 提示:
* `1 <= k <= jobs.length <= 12`
* <code>1 <= jobs[i] <= 10<sup>7</sup></code>

## 题解 (Python)

### 1. 题解
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
