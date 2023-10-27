# 729. My Calendar I
You are implementing a program to use as your calendar. We can add a new event if adding the event will not cause a **double booking**.

A **double booking** happens when two events have some non-empty intersection (i.e., some moment is common to both events.).

The event can be represented as a pair of integers `start` and `end` that represents a booking on the half-open interval `[start, end)`, the range of real numbers `x` such that `start <= x < end`.

Implement the `MyCalendar` class:

* `MyCalendar()` Initializes the calendar object.
* `boolean book(int start, int end)` Returns `true` if the event can be added to the calendar successfully without causing a **double booking**. Otherwise, return `false` and do not add the event to the calendar.

#### Example 1:
<pre>
<strong>Input:</strong>
["MyCalendar", "book", "book", "book"]
[[], [10, 20], [15, 25], [20, 30]]
<strong>Output:</strong>
[null, true, false, true]
<strong>Explanation:</strong>
MyCalendar myCalendar = new MyCalendar();
myCalendar.book(10, 20); // return True
myCalendar.book(15, 25); // return False, It can not be booked because time 15 is already booked by another event.
myCalendar.book(20, 30); // return True, The event can be booked, as the first event takes every time less than 20, but not including 20.
</pre>

#### Constraints:
* <code>0 <= start < end <= 10<sup>9</sup></code>
* At most `1000` calls will be made to `book`.

## Solutions (Python)

### 1. Solution
```Python
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
```
