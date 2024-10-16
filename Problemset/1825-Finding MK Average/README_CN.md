# 1825. 求出 MK 平均值
给你两个整数 `m` 和 `k` ，以及数据流形式的若干整数。你需要实现一个数据结构，计算这个数据流的 **MK 平均值** 。

**MK 平均值** 按照如下步骤计算：

1. 如果数据流中的整数少于 `m` 个，**MK 平均值** 为 `-1` ，否则将数据流中最后 `m` 个元素拷贝到一个独立的容器中。
2. 从这个容器中删除最小的 `k` 个数和最大的 `k` 个数。
3. 计算剩余元素的平均值，并 **向下取整到最近的整数** 。

请你实现 `MKAverage` 类：

* `MKAverage(int m, int k)` 用一个空的数据流和两个整数 `m` 和 `k` 初始化 **MKAverage** 对象。
* `void addElement(int num)` 往数据流中插入一个新的元素 `num` 。
* `int calculateMKAverage()` 对当前的数据流计算并返回 **MK 平均数** ，结果需 **向下取整到最近的整数** 。

#### 示例 1:
<pre>
<strong>输入:</strong>
["MKAverage", "addElement", "addElement", "calculateMKAverage", "addElement", "calculateMKAverage", "addElement", "addElement", "addElement", "calculateMKAverage"]
[[3, 1], [3], [1], [], [10], [], [5], [5], [5], []]
<strong>输出:</strong>
[null, null, null, -1, null, 3, null, null, null, 5]
<strong>解释:</strong>
MKAverage obj = new MKAverage(3, 1);
obj.addElement(3);        // 当前元素为 [3]
obj.addElement(1);        // 当前元素为 [3,1]
obj.calculateMKAverage(); // 返回 -1 ，因为 m = 3 ，但数据流中只有 2 个元素
obj.addElement(10);       // 当前元素为 [3,1,10]
obj.calculateMKAverage(); // 最后 3 个元素为 [3,1,10]
                          // 删除最小以及最大的 1 个元素后，容器为 [3]
                          // [3] 的平均值等于 3/1 = 3 ，故返回 3
obj.addElement(5);        // 当前元素为 [3,1,10,5]
obj.addElement(5);        // 当前元素为 [3,1,10,5,5]
obj.addElement(5);        // 当前元素为 [3,1,10,5,5,5]
obj.calculateMKAverage(); // 最后 3 个元素为 [5,5,5]
                          // 删除最小以及最大的 1 个元素后，容器为 [5]
                          // [5] 的平均值等于 5/1 = 5 ，故返回 5
</pre>

#### 提示:
* <code>3 <= m <= 10<sup>5</sup></code>
* `1 <= k*2 < m`
* <code>1 <= num <= 10<sup>5</sup></code>
* `addElement` 与 `calculateMKAverage` 总操作次数不超过 <code>10<sup>5</sup></code> 次。

## 题解 (Python)

### 1. 题解
```Python
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
```
