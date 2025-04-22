from functools import cache


class Solution:
    @cache
    def earliestAndLatest(self, n: int, firstPlayer: int, secondPlayer: int) -> List[int]:
        if firstPlayer - 1 == n - secondPlayer:
            return [1, 1]
        if firstPlayer - 1 > n - secondPlayer:
            return self.earliestAndLatest(n, n + 1 - secondPlayer, n + 1 - firstPlayer)

        mid = (n + 1) // 2
        earliest = 5
        latest = 2

        for i in range(firstPlayer):
            if secondPlayer <= mid:
                rangej = range(secondPlayer - firstPlayer)
            elif n % 2 == 0:
                rangej = range(secondPlayer - mid - 1, n - firstPlayer - mid)
            else:
                rangej = range(secondPlayer - mid, n - firstPlayer - mid + 1)
            for j in rangej:
                early, late = self.earliestAndLatest(mid, i + 1, i + j + 2)
                earliest = min(earliest, early + 1)
                latest = max(latest, late + 1)

        return [earliest, latest]
