from sortedcontainers import SortedList, SortedKeyList


class Solution:
    def oddEvenJumps(self, arr: List[int]) -> int:
        n = len(arr)
        asc = SortedList()
        desc = SortedKeyList(key=lambda x: [-x[0], x[1]])
        dp = [[False, False] for _ in range(n)]
        dp[n - 1] = [True, True]
        ret = 0

        for i in range(n)[::-1]:
            j = asc.bisect_left([arr[i], 0])
            j = asc[j][1] if j < len(asc) else n
            dp[i][0] |= j < n and dp[j][1]
            j = desc.bisect_left([arr[i], 0])
            j = desc[j][1] if j < len(desc) else n
            dp[i][1] |= j < n and dp[j][0]

            asc.add([arr[i], i])
            desc.add([arr[i], i])

            if dp[i][0]:
                ret += 1

        return ret
