# 2286. Booking Concert Tickets in Groups
A concert hall has `n` rows numbered from `0` to `n - 1`, each with `m` seats, numbered from `0` to `m - 1`. You need to design a ticketing system that can allocate seats in the following cases:
* If a group of `k` spectators can sit **together** in a row.
* If **every** member of a group of `k` spectators can get a seat. They may or **may not** sit together.

Note that the spectators are very picky. Hence:
* They will book seats only if each member of their group can get a seat with row number **less than or equal** to `maxRow`. `maxRow` can **vary** from group to group.
* In case there are multiple rows to choose from, the row with the **smallest** number is chosen. If there are multiple seats to choose in the same row, the seat with the **smallest** number is chosen.

Implement the `BookMyShow` class:
* `BookMyShow(int n, int m)` Initializes the object with `n` as number of rows and `m` as number of seats per row.
* `int[] gather(int k, int maxRow)` Returns an array of length `2` denoting the row and seat number (respectively) of the **first seat** being allocated to the `k` members of the group, who must sit **together**. In other words, it returns the smallest possible `r` and `c` such that all `[c, c + k - 1]` seats are valid and empty in row `r`, and `r <= maxRow`. Returns `[]` in case it is **not possible** to allocate seats to the group.
* `boolean scatter(int k, int maxRow)` Returns `true` if all `k` members of the group can be allocated seats in rows `0` to `maxRow`, who may or **may not** sit together. If the seats can be allocated, it allocates `k` seats to the group with the **smallest** row numbers, and the smallest possible seat numbers in each row. Otherwise, returns `false`.

#### Example 1:
<pre>
<strong>Input:</strong>
["BookMyShow", "gather", "gather", "scatter", "scatter"]
[[2, 5], [4, 0], [2, 0], [5, 1], [5, 1]]
<strong>Output:</strong>
[null, [0, 0], [], true, false]
<strong>Explanation:</strong>
BookMyShow bms = new BookMyShow(2, 5); // There are 2 rows with 5 seats each
bms.gather(4, 0); // return [0, 0]
                  // The group books seats [0, 3] of row 0.
bms.gather(2, 0); // return []
                  // There is only 1 seat left in row 0,
                  // so it is not possible to book 2 consecutive seats.
bms.scatter(5, 1); // return True
                   // The group books seat 4 of row 0 and seats [0, 3] of row 1.
bms.scatter(5, 1); // return False
                   // There is only one seat left in the hall.
</pre>

#### Constraints:
* <code>1 <= n <= 5 * 10<sup>4</sup></code>
* <code>1 <= m, k <= 10<sup>9</sup></code>
* `0 <= maxRow <= n - 1`
* At most <code>5 * 10<sup>4</sup></code> calls **in total** will be made to `gather` and `scatter`.

## Solutions (Python)

### 1. Solution
```Python
class BookMyShow:

    def __init__(self, n: int, m: int):
        self.m = m
        self.size = 1
        while self.size < n:
            self.size *= 2
        self.maxtree = [0] * (2 * self.size)
        self.sumtree = [0] * (2 * self.size)
        for i in range(n):
            self.maxtree[self.size + i] = m
            self.sumtree[self.size + i] = m
        for i in range(self.size - 1, 0, -1):
            self.maxtree[i] = max(self.maxtree[2 * i], self.maxtree[2 * i + 1])
            self.sumtree[i] = self.sumtree[2 * i] + self.sumtree[2 * i + 1]

    def gather(self, k: int, maxRow: int, i: int = 1) -> List[int]:
        if self.maxtree[i] < k or i - self.size > maxRow:
            return []

        if i >= self.size:
            self.maxtree[i] -= k
            self.sumtree[i] -= k
            return [i - self.size, self.m - self.maxtree[i] - k]

        ret = self.gather(k, maxRow, 2 * i)
        if ret == []:
            ret = self.gather(k, maxRow, 2 * i + 1)
        if ret != []:
            self.maxtree[i] = max(self.maxtree[2 * i], self.maxtree[2 * i + 1])
            self.sumtree[i] -= k

        return ret

    def scatter(self, k: int, maxRow: int, i: int = 1) -> bool:
        if self.sumtree[i] < k or i - self.size > maxRow:
            return False

        if i >= self.size:
            self.maxtree[i] -= k
            self.sumtree[i] -= k
            return True

        if self.sumtree[2 * i] >= k:
            if self.scatter(k, maxRow, 2 * i):
                self.maxtree[i] = max(
                    self.maxtree[2 * i], self.maxtree[2 * i + 1])
                self.sumtree[i] -= k
                return True
        elif self.scatter(k - self.sumtree[2 * i], maxRow, 2 * i + 1):
            self.maxtree[2 * i] = 0
            self.sumtree[2 * i] = 0
            self.maxtree[i] = self.maxtree[2 * i + 1]
            self.sumtree[i] = self.sumtree[2 * i + 1]
            return True

        return False


# Your BookMyShow object will be instantiated and called as such:
# obj = BookMyShow(n, m)
# param_1 = obj.gather(k,maxRow)
# param_2 = obj.scatter(k,maxRow)
```
