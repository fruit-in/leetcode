# 519. Random Flip Matrix
There is an `m x n` binary grid `matrix` with all the values set `0` initially. Design an algorithm to randomly pick an index `(i, j)` where `matrix[i][j] == 0` and flips it to `1`. All the indices `(i, j)` where `matrix[i][j] == 0` should be equally likely to be returned.

Optimize your algorithm to minimize the number of calls made to the **built-in** random function of your language and optimize the time and space complexity.

Implement the `Solution` class:

* `Solution(int m, int n)` Initializes the object with the size of the binary matrix `m` and `n`.
* `int[] flip()` Returns a random index `[i, j]` of the matrix where `matrix[i][j] == 0` and flips it to `1`.
* `void reset()` Resets all the values of the matrix to be `0`.

#### Example 1:
<pre>
<strong>Input:</strong>
["Solution", "flip", "flip", "flip", "reset", "flip"]
[[3, 1], [], [], [], [], []]
<strong>Output:</strong>
[null, [1, 0], [2, 0], [0, 0], null, [2, 0]]
<strong>Explanation:</strong>
Solution solution = new Solution(3, 1);
solution.flip();  // return [1, 0], [0,0], [1,0], and [2,0] should be equally likely to be returned.
solution.flip();  // return [2, 0], Since [1,0] was returned, [2,0] and [0,0]
solution.flip();  // return [0, 0], Based on the previously returned indices, only [0,0] can be returned.
solution.reset(); // All the values are reset to 0 and can be returned.
solution.flip();  // return [2, 0], [0,0], [1,0], and [2,0] should be equally likely to be returned.
</pre>

#### Constraints:
* <code>1 <= m, n <= 10<sup>4</sup></code>
* There will be at least one free cell for each call to `flip`.
* At most `1000` calls will be made to `flip` and `reset`.

## Solutions (Python)

### 1. Solution
```Python
from random import randint


class Solution:

    def __init__(self, m: int, n: int):
        self.m = m
        self.n = n
        self.remain = m * n
        self.mapping = {}

    def flip(self) -> List[int]:
        self.remain -= 1
        i = randint(0, self.remain)
        self.mapping[i], self.mapping[self.remain] = self.mapping.get(
            self.remain, self.remain), self.mapping.get(i, i)

        return [self.mapping[self.remain] // self.n, self.mapping[self.remain] % self.n]

    def reset(self) -> None:
        self.remain = self.m * self.n
        self.mapping.clear()


# Your Solution object will be instantiated and called as such:
# obj = Solution(m, n)
# param_1 = obj.flip()
# obj.reset()
```
