class Solution:
    def maxHappyGroups(self, batchSize: int, groups: List[int]) -> int:
        @cache
        def dfs(count: Tuple[int]) -> int:
            count = list(count)
            remain = (total - sum(i * count[i]
                      for i in range(1, batchSize))) % batchSize
            ret = 0

            for i in range(1, batchSize):
                if count[i] == 0:
                    continue

                count[i] -= 1
                if remain == 0:
                    ret = max(ret, 1 + dfs(tuple(count)))
                else:
                    ret = max(ret, dfs(tuple(count)))
                count[i] += 1

            return ret

        count = [0] * batchSize
        total = 0

        for x in groups:
            count[x % batchSize] += 1
            total += x % batchSize

        return count[0] + dfs(tuple(count))
