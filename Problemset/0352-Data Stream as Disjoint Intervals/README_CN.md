# 352. 将数据流变为多个不相交区间
 给你一个由非负整数 <code>a<sub>1</sub>, a<sub>2</sub>, ..., a<sub>n</sub></code> 组成的数据流输入，请你将到目前为止看到的数字总结为不相交的区间列表。

实现 `SummaryRanges` 类：

* `SummaryRanges()` 使用一个空数据流初始化对象。
* `void addNum(int val)` 向数据流中加入整数 `val` 。
* `int[][] getIntervals()` 以不相交区间 <code>[start<sub>i</sub>, end<sub>i</sub>]</code> 的列表形式返回对数据流中整数的总结。

#### 示例 1:
<pre>
<strong>输入:</strong>
["SummaryRanges", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals", "addNum", "getIntervals"]
[[], [1], [], [3], [], [7], [], [2], [], [6], []]
<strong>输出:</strong>
[null, null, [[1, 1]], null, [[1, 1], [3, 3]], null, [[1, 1], [3, 3], [7, 7]], null, [[1, 3], [7, 7]], null, [[1, 3], [6, 7]]]
<strong>解释:</strong>
SummaryRanges summaryRanges = new SummaryRanges();
summaryRanges.addNum(1);      // arr = [1]
summaryRanges.getIntervals(); // 返回 [[1, 1]]
summaryRanges.addNum(3);      // arr = [1, 3]
summaryRanges.getIntervals(); // 返回 [[1, 1], [3, 3]]
summaryRanges.addNum(7);      // arr = [1, 3, 7]
summaryRanges.getIntervals(); // 返回 [[1, 1], [3, 3], [7, 7]]
summaryRanges.addNum(2);      // arr = [1, 2, 3, 7]
summaryRanges.getIntervals(); // 返回 [[1, 3], [7, 7]]
summaryRanges.addNum(6);      // arr = [1, 2, 3, 6, 7]
summaryRanges.getIntervals(); // 返回 [[1, 3], [6, 7]]
</pre>

#### 提示:
* <code>0 <= value <= 10<sup>4</sup></code>
* 最多调用 `addNum` 和 `getIntervals` 方法 <code>3 * 10<sup>4</sup></code> 次

**进阶：**如果存在大量合并，并且与数据流的大小相比，不相交区间的数量很小，该怎么办?

## 题解 (Python)

### 1. 题解
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
