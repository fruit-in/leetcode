class BookMyShow:

    def __init__(self, n: int, m: int):
        self.m = m
        self.size = 1
        while self.size < n:
            self.size *= 2
        self.maxtree = [0] * (2 * self.size)
        self.sumtree = [0] * (2 * self.size)
        for i in range(n):
            self.maxtree[self.size + i] = m
            self.sumtree[self.size + i] = m
        for i in range(self.size - 1, 0, -1):
            self.maxtree[i] = max(self.maxtree[2 * i], self.maxtree[2 * i + 1])
            self.sumtree[i] = self.sumtree[2 * i] + self.sumtree[2 * i + 1]

    def gather(self, k: int, maxRow: int, i: int = 1) -> List[int]:
        if self.maxtree[i] < k or i - self.size > maxRow:
            return []

        if i >= self.size:
            self.maxtree[i] -= k
            self.sumtree[i] -= k
            return [i - self.size, self.m - self.maxtree[i] - k]

        ret = self.gather(k, maxRow, 2 * i)
        if ret == []:
            ret = self.gather(k, maxRow, 2 * i + 1)
        if ret != []:
            self.maxtree[i] = max(self.maxtree[2 * i], self.maxtree[2 * i + 1])
            self.sumtree[i] -= k

        return ret

    def scatter(self, k: int, maxRow: int, i: int = 1) -> bool:
        if self.sumtree[i] < k or i - self.size > maxRow:
            return False

        if i >= self.size:
            self.maxtree[i] -= k
            self.sumtree[i] -= k
            return True

        if self.sumtree[2 * i] >= k:
            if self.scatter(k, maxRow, 2 * i):
                self.maxtree[i] = max(
                    self.maxtree[2 * i], self.maxtree[2 * i + 1])
                self.sumtree[i] -= k
                return True
        elif self.scatter(k - self.sumtree[2 * i], maxRow, 2 * i + 1):
            self.maxtree[2 * i] = 0
            self.sumtree[2 * i] = 0
            self.maxtree[i] = self.maxtree[2 * i + 1]
            self.sumtree[i] = self.sumtree[2 * i + 1]
            return True

        return False


# Your BookMyShow object will be instantiated and called as such:
# obj = BookMyShow(n, m)
# param_1 = obj.gather(k,maxRow)
# param_2 = obj.scatter(k,maxRow)
