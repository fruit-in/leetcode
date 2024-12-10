from sortedcontainers import SortedList


class Solution:
    def findLatestStep(self, arr: List[int], m: int) -> int:
        if len(arr) == m:
            return m

        n = len(arr)
        groups = SortedList([(1, n + 1)])

        for step in range(n - 1, -1, -1):
            i = groups.bisect_left((arr[step], n + 2)) - 1
            left = (groups[i][0], arr[step])
            right = (arr[step] + 1, groups[i][1])
            groups.pop(i)
            if left[1] - left[0] == m or right[1] - right[0] == m:
                return step
            if left[0] < left[1]:
                groups.add(left)
            if right[0] < right[1]:
                groups.add(right)

        return -1
