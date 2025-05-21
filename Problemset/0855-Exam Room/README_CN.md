# 855. 考场就座
在考场里，有 `n` 个座位排成一行，编号为 `0` 到 `n - 1`。

当学生进入考场后，他必须坐在离最近的人最远的座位上。如果有多个这样的座位，他会坐在编号最小的座位上。(另外，如果考场里没有人，那么学生就坐在 `0` 号座位上。)

设计一个模拟所述考场的类。

实现 `ExamRoom` 类：
* `ExamRoom(int n)` 用座位的数量 `n` 初始化考场对象。
* `int seat()` 返回下一个学生将会入座的座位编号。
* `void leave(int p)` 指定坐在座位 `p` 的学生将离开教室。保证座位 `p` 上会有一位学生。

#### 示例 1:
<pre>
<strong>输入:</strong>
["ExamRoom", "seat", "seat", "seat", "seat", "leave", "seat"]
[[10], [], [], [], [], [4], []]
<strong>输出:</strong>
[null, 0, 9, 4, 2, null, 5]
<strong>解释:</strong>
ExamRoom examRoom = new ExamRoom(10);
examRoom.seat(); // 返回 0，房间里没有人，学生坐在 0 号座位。
examRoom.seat(); // 返回 9，学生最后坐在 9 号座位。
examRoom.seat(); // 返回 4，学生最后坐在 4 号座位。
examRoom.seat(); // 返回 2，学生最后坐在 2 号座位。
examRoom.leave(4);
examRoom.seat(); // 返回 5，学生最后坐在 5 号座位。
</pre>

#### 提示:
1. <code>1 <= n <= 10<sup>9</sup></code>
2. 保证有学生正坐在座位 `p` 上。
3. `seat` 和 `leave` 最多被调用 <code>10<sup>4</sup></code> 次。

## 题解 (Python)

### 1. 题解
```Python
class ExamRoom:

    def __init__(self, n: int):
        self.n = n
        self.seatsheap = [(-inf, 0, -inf, inf)]
        self.used = SortedList([-inf, inf])

    def seat(self) -> int:
        while self.seatsheap:
            _, p, left, right = heappop(self.seatsheap)
            i = self.used.bisect_left(p)
            if self.used[i] != p and self.used[i - 1] == left and self.used[i] == right:
                self.used.add(p)
                if left == -inf and p != 0:
                    heappush(self.seatsheap, (-p, 0, -inf, p))
                elif p != 0:
                    dist = (p - left) // 2
                    heappush(self.seatsheap, (-dist, left + dist, left, p))
                if right == inf and p != self.n - 1:
                    heappush(self.seatsheap,
                             (p - self.n + 1, self.n - 1, p, inf))
                elif p != self.n - 1:
                    dist = (right - p) // 2
                    heappush(self.seatsheap, (-dist, p + dist, p, right))

                return p

    def leave(self, p: int) -> None:
        i = self.used.bisect_left(p)
        self.used.pop(i)
        left, right = self.used[i - 1], self.used[i]
        if left == -inf:
            heappush(self.seatsheap, (-right, 0, -inf, right))
        elif right == inf:
            heappush(self.seatsheap, (left - self.n + 1, self.n - 1, left, inf))
        else:
            dist = (right - left) // 2
            heappush(self.seatsheap, (-dist, left + dist, left, right))


# Your ExamRoom object will be instantiated and called as such:
# obj = ExamRoom(n)
# param_1 = obj.seat()
# obj.leave(p)
```
