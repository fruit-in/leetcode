from sortedcontainers import SortedList


class MKAverage:

    def __init__(self, m: int, k: int):
        self.m = m
        self.k = k
        self.sum = 0
        self.queue = collections.deque()
        self.container = SortedList()

    def addElement(self, num: int) -> None:
        if len(self.queue) < self.m:
            self.queue.append(num)
            self.container.add(num)
            if len(self.queue) == self.m:
                self.sum = sum(self.container[self.k:-self.k])
            return

        if num < self.container[self.k]:
            self.sum -= self.container[-self.k - 1]
            self.container.add(num)
            self.sum += self.container[self.k]
        elif num < self.container[-self.k]:
            self.sum += num
            self.container.add(num)
            self.sum -= self.container[-self.k - 1]
        else:
            self.container.add(num)
        self.queue.append(num)

        if self.queue[0] < self.container[self.k]:
            self.sum += self.container[-self.k - 1]
            self.sum -= self.container[self.k]
        elif self.queue[0] < self.container[-self.k - 1]:
            self.sum += self.container[-self.k - 1]
            self.sum -= self.queue[0]
        self.container.discard(self.queue.popleft())

    def calculateMKAverage(self) -> int:
        if len(self.queue) < self.m:
            return -1

        return self.sum // (self.m - self.k * 2)


# Your MKAverage object will be instantiated and called as such:
# obj = MKAverage(m, k)
# obj.addElement(num)
# param_2 = obj.calculateMKAverage()
