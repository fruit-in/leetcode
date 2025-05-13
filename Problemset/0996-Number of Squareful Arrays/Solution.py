class Solution:
    def numSquarefulPerms(self, nums: List[int]) -> int:
        n = len(nums)
        neighbors = {x: set() for x in nums} | {None: set(nums)}
        remain = {x: 0 for x in nums}
        ret = 0

        for i in range(n):
            x = nums[i]
            remain[x] += 1
            for j in range(i + 1, n):
                y = nums[j]
                if int(math.sqrt(x + y)) ** 2 == x + y:
                    neighbors[x].add(y)
                    neighbors[y].add(x)

        def dfs(i: int, prev: Optional[int]) -> None:
            nonlocal ret
            if i == n:
                ret += 1
                return

            for x in neighbors[prev]:
                if remain[x] > 0:
                    remain[x] -= 1
                    dfs(i + 1, x)
                    remain[x] += 1

        dfs(0, None)

        return ret
