class Solution:
    def maxRunTime(self, n: int, batteries: List[int]) -> int:
        return bisect_right(range(sum(batteries) // n + 1), -n, key=lambda m: -(sum(min(m, b) for b in batteries) // m)) - 1
