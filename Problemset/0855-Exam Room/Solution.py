class ExamRoom:

    def __init__(self, n: int):
        self.n = n
        self.seatsheap = [(-inf, 0, -inf, inf)]
        self.used = SortedList([-inf, inf])

    def seat(self) -> int:
        while self.seatsheap:
            _, p, left, right = heappop(self.seatsheap)
            i = self.used.bisect_left(p)
            if self.used[i] != p and self.used[i - 1] == left and self.used[i] == right:
                self.used.add(p)
                if left == -inf and p != 0:
                    heappush(self.seatsheap, (-p, 0, -inf, p))
                elif p != 0:
                    dist = (p - left) // 2
                    heappush(self.seatsheap, (-dist, left + dist, left, p))
                if right == inf and p != self.n - 1:
                    heappush(self.seatsheap,
                             (p - self.n + 1, self.n - 1, p, inf))
                elif p != self.n - 1:
                    dist = (right - p) // 2
                    heappush(self.seatsheap, (-dist, p + dist, p, right))

                return p

    def leave(self, p: int) -> None:
        i = self.used.bisect_left(p)
        self.used.pop(i)
        left, right = self.used[i - 1], self.used[i]
        if left == -inf:
            heappush(self.seatsheap, (-right, 0, -inf, right))
        elif right == inf:
            heappush(self.seatsheap, (left - self.n + 1, self.n - 1, left, inf))
        else:
            dist = (right - left) // 2
            heappush(self.seatsheap, (-dist, left + dist, left, right))


# Your ExamRoom object will be instantiated and called as such:
# obj = ExamRoom(n)
# param_1 = obj.seat()
# obj.leave(p)
