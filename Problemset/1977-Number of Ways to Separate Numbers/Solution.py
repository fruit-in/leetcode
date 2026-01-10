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
