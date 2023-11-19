from sortedcontainers import SortedList


class SummaryRanges:

    def __init__(self):
        self.intervals = SortedList()
        self.nums = set()

    def addNum(self, value: int) -> None:
        if value in self.nums:
            return

        i = self.intervals.bisect_left([value, value])
        self.nums.add(value)

        if value - 1 in self.nums and value + 1 in self.nums:
            x, _ = self.intervals.pop(i - 1)
            _, y = self.intervals.pop(i - 1)
            self.intervals.add([x, y])
        elif value - 1 in self.nums:
            x, _ = self.intervals.pop(i - 1)
            self.intervals.add([x, value])
        elif value + 1 in self.nums:
            _, y = self.intervals.pop(i)
            self.intervals.add([value, y])
        else:
            self.intervals.add([value, value])

    def getIntervals(self) -> List[List[int]]:
        return list(self.intervals)


# Your SummaryRanges object will be instantiated and called as such:
# obj = SummaryRanges()
# obj.addNum(value)
# param_2 = obj.getIntervals()
