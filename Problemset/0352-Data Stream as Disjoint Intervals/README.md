# 352. Data Stream as Disjoint Intervals
Given a data stream input of non-negative integers <code>a<sub>1</sub>, a<sub>2</sub>, ..., a<sub>n</sub></code>, summarize the numbers seen so far as a list of disjoint intervals.

Implement the `SummaryRanges` class:

* `SummaryRanges()` Initializes the object with an empty stream.
* `void addNum(int value)` Adds the integer `value` to the stream.
* `int[][] getIntervals()` Returns a summary of the integers in the stream currently as a list of disjoint intervals <code>[start<sub>i</sub>, end<sub>i</sub>]</code>. The answer should be sorted by <code>start<sub>i</sub></code>.

#### Example 1:
<pre>
<strong>Input:</strong>
["SummaryRanges", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals"]
[[], [1], [], [3], [], [7], [], [2], [], [6], []]
<strong>Output:</strong>
[null, null, [[1, 1]], null, [[1, 1], [3, 3]], null, [[1, 1], [3, 3], [7, 7]], null, [[1, 3], [7, 7]], null, [[1, 3], [6, 7]]]
<strong>Explanation:</strong>
SummaryRanges summaryRanges = new SummaryRanges();
summaryRanges.addNum(1);      // arr = [1]
summaryRanges.getIntervals(); // return [[1, 1]]
summaryRanges.addNum(3);      // arr = [1, 3]
summaryRanges.getIntervals(); // return [[1, 1], [3, 3]]
summaryRanges.addNum(7);      // arr = [1, 3, 7]
summaryRanges.getIntervals(); // return [[1, 1], [3, 3], [7, 7]]
summaryRanges.addNum(2);      // arr = [1, 2, 3, 7]
summaryRanges.getIntervals(); // return [[1, 3], [7, 7]]
summaryRanges.addNum(6);      // arr = [1, 2, 3, 6, 7]
summaryRanges.getIntervals(); // return [[1, 3], [6, 7]]
</pre>

#### Constraints:
* <code>0 <= value <= 10<sup>4</sup></code>
* At most <code>3 * 10<sup>4</sup></code> calls will be made to `addNum` and `getIntervals`.
* At most <code>10<sup>2</sup></code> calls will be made to `getIntervals`.

**Follow up:** What if there are lots of merges and the number of disjoint intervals is small compared to the size of the data stream?

## Solutions (Python)

### 1. Solution
```Python
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
```
