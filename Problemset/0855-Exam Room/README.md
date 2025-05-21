# 855. Exam Room
There is an exam room with `n` seats in a single row labeled from `0` to `n - 1`.

When a student enters the room, they must sit in the seat that maximizes the distance to the closest person. If there are multiple such seats, they sit in the seat with the lowest number. If no one is in the room, then the student sits at seat number `0`.

Design a class that simulates the mentioned exam room.

Implement the `ExamRoom` class:
* `ExamRoom(int n)` Initializes the object of the exam room with the number of the seats `n`.
* `int seat()` Returns the label of the seat at which the next student will set.
* `void leave(int p)` Indicates that the student sitting at seat `p` will leave the room. It is guaranteed that there will be a student sitting at seat `p`.

#### Example 1:
<pre>
<strong>Input:</strong>
["ExamRoom", "seat", "seat", "seat", "seat", "leave", "seat"]
[[10], [], [], [], [], [4], []]
<strong>Output:</strong>
[null, 0, 9, 4, 2, null, 5]
<strong>Explanation:</strong>
ExamRoom examRoom = new ExamRoom(10);
examRoom.seat(); // return 0, no one is in the room, then the student sits at seat number 0.
examRoom.seat(); // return 9, the student sits at the last seat number 9.
examRoom.seat(); // return 4, the student sits at the last seat number 4.
examRoom.seat(); // return 2, the student sits at the last seat number 2.
examRoom.leave(4);
examRoom.seat(); // return 5, the student sits at the last seat number 5.
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>9</sup></code>
* It is guaranteed that there is a student sitting at seat `p`.
* At most <code>10<sup>4</sup></code> calls will be made to `seat` and `leave`.

## Solutions (Python)

### 1. Solution
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
