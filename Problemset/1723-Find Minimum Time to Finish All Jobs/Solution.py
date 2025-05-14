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
