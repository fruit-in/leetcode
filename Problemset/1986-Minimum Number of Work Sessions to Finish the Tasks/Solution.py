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
