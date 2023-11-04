# 295. 数据流的中位数
**中位数**是有序整数列表中的中间值。如果列表的大小是偶数，则没有中间值，中位数是两个中间值的平均值。

* 例如 `arr = [2,3,4]` 的中位数是 `3` 。
* 例如 `arr = [2,3]` 的中位数是 `(2 + 3) / 2 = 2.5` 。

实现 MedianFinder 类:

* `MedianFinder()` 初始化 `MedianFinder` 对象。
* `void addNum(int num)` 将数据流中的整数 `num` 添加到数据结构中。
* `double findMedian()` 返回到目前为止所有元素的中位数。与实际答案相差 <code>10<sup>-5</sup></code> 以内的答案将被接受。

#### 示例 1:
<pre>
<strong>输入:</strong>
["MedianFinder", "addNum", "addNum", "findMedian", "addNum", "findMedian"]
[[], [1], [2], [], [3], []]
<strong>输出:</strong>
[null, null, null, 1.5, null, 2.0]
<strong>解释:</strong>
MedianFinder medianFinder = new MedianFinder();
medianFinder.addNum(1);    // arr = [1]
medianFinder.addNum(2);    // arr = [1, 2]
medianFinder.findMedian(); // 返回 1.5 ((1 + 2) / 2)
medianFinder.addNum(3);    // arr[1, 2, 3]
medianFinder.findMedian(); // return 2.0
</pre>

#### 提示:
* <code>-10<sup>5</sup> <= num <= 10<sup>5</sup></code>
* 在调用 `findMedian` 之前，数据结构中至少有一个元素
* 最多 <code>5 * 10<sup>4</sup></code> 次调用 `addNum` 和 `findMedian`

## 题解 (Python)

### 1. 题解
```Python
from sortedcontainers import SortedList


class MedianFinder:

    def __init__(self):
        self.nums = SortedList()

    def addNum(self, num: int) -> None:
        self.nums.add(num)

    def findMedian(self) -> float:
        n = len(self.nums)

        if n % 2 == 1:
            return self.nums[n // 2]
        else:
            return (self.nums[n // 2 - 1] + self.nums[n // 2]) / 2


# Your MedianFinder object will be instantiated and called as such:
# obj = MedianFinder()
# obj.addNum(num)
# param_2 = obj.findMedian()
```
