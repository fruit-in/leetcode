from sortedcontainers import SortedList


class MyCalendar:

    def __init__(self):
        self.calendar = SortedList([])

    def book(self, start: int, end: int) -> bool:
        i = self.calendar.bisect_left((start, end))

        if i < len(self.calendar) and end > self.calendar[i][0]:
            return False
        if i > 0 and start < self.calendar[i - 1][1]:
            return False

        self.calendar.add((start, end))

        return True


# Your MyCalendar object will be instantiated and called as such:
# obj = MyCalendar()
# param_1 = obj.book(start,end)
