class Solution:
    def maximizeWin(self, prizePositions: List[int], k: int) -> int:
        leftMax = 0
        ret = 0

        for i in range(len(prizePositions)):
            j = bisect.bisect(prizePositions, prizePositions[i] + k)
            ret = max(ret, leftMax + j - i)
            j = bisect.bisect_left(prizePositions, prizePositions[i] - k)
            leftMax = max(leftMax, i - j + 1)

        return ret
