# 2286. 以组为单位订音乐会的门票
一个音乐会总共有 `n` 排座位，编号从 `0` 到 `n - 1` ，每一排有 `m` 个座椅，编号为 `0` 到 `m - 1` 。你需要设计一个买票系统，针对以下情况进行座位安排：
* 同一组的 `k` 位观众坐在 **同一排座位，且座位连续** 。
* `k` 位观众中 **每一位** 都有座位坐，但他们 **不一定** 坐在一起。

由于观众非常挑剔，所以：
* 只有当一个组里所有成员座位的排数都 **小于等于** `maxRow` ，这个组才能订座位。每一组的 `maxRow` 可能 **不同** 。
* 如果有多排座位可以选择，优先选择 **最小** 的排数。如果同一排中有多个座位可以坐，优先选择号码 **最小** 的。

请你实现 `BookMyShow` 类：
* `BookMyShow(int n, int m)` ，初始化对象，`n` 是排数，`m` 是每一排的座位数。
* `int[] gather(int k, int maxRow)` 返回长度为 `2` 的数组，表示 `k` 个成员中 **第一个座位** 的排数和座位编号，这 `k` 位成员必须坐在 **同一排座位，且座位连续** 。换言之，返回最小可能的 `r` 和 `c` 满足第 `r` 排中 `[c, c + k - 1]` 的座位都是空的，且 `r <= maxRow` 。如果 **无法** 安排座位，返回 `[]` 。
* `boolean scatter(int k, int maxRow)` 如果组里所有 `k` 个成员 **不一定** 要坐在一起的前提下，都能在第 `0` 排到第 `maxRow` 排之间找到座位，那么请返回 `true` 。这种情况下，每个成员都优先找排数 **最小** ，然后是座位编号最小的座位。如果不能安排所有 `k` 个成员的座位，请返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong>
["BookMyShow", "gather", "gather", "scatter", "scatter"]
[[2, 5], [4, 0], [2, 0], [5, 1], [5, 1]]
<strong>输出:</strong>
[null, [0, 0], [], true, false]
<strong>解释:</strong>
BookMyShow bms = new BookMyShow(2, 5); // 总共有 2 排，每排 5 个座位。
bms.gather(4, 0); // 返回 [0, 0]
                  // 这一组安排第 0 排 [0, 3] 的座位。
bms.gather(2, 0); // 返回 []
                  // 第 0 排只剩下 1 个座位。
                  // 所以无法安排 2 个连续座位。
bms.scatter(5, 1); // 返回 True
                   // 这一组安排第 0 排第 4 个座位和第 1 排 [0, 3] 的座位。
bms.scatter(5, 1); // 返回 False
                   // 总共只剩下 1 个座位。
</pre>

#### 提示:
* <code>1 <= n <= 5 * 10<sup>4</sup></code>
* <code>1 <= m, k <= 10<sup>9</sup></code>
* `0 <= maxRow <= n - 1`
* `gather` 和 `scatter` **总** 调用次数不超过 <code>5 * 10<sup>4</sup></code> 次。

## 题解 (Python)

### 1. 题解
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
