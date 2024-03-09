from sortedcontainers import SortedList


class MyCalendarTwo:

    def __init__(self):
        self.events = []
        self.double = SortedList()

    def book(self, start: int, end: int) -> bool:
        i = self.double.bisect_left((start, 1000000007))
        if i > 0 and self.double[i - 1][1] > start:
            return False
        if i < len(self.double) and self.double[i][0] < end:
            return False

        for s, e in self.events:
            if start < e and end > s:
                self.double.add((max(start, s), min(end, e)))
        self.events.append((start, end))

        return True


# Your MyCalendarTwo object will be instantiated and called as such:
# obj = MyCalendarTwo()
# param_1 = obj.book(start,end)
