from sortedcontainers import SortedList


class Solution:
    def longestObstacleCourseAtEachPosition(self, obstacles: List[int]) -> List[int]:
        n = len(obstacles)
        sl = SortedList([(0, 0)])
        ans = [1] * n

        for i in range(n):
            j = sl.bisect_left((obstacles[i], n)) - 1
            ans[i] = sl[j][1] + 1

            if j + 1 < len(sl) and sl[j + 1][1] == ans[i]:
                sl.pop(j + 1)
            if sl[j][0] == obstacles[i]:
                sl.pop(j)

            sl.add((obstacles[i], ans[i]))

        return ans
