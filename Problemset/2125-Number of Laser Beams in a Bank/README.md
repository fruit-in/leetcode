# 2125. Number of Laser Beams in a Bank
Anti-theft security devices are activated inside a bank. You are given a **0-indexed** binary string array `bank` representing the floor plan of the bank, which is an `m x n` 2D matrix. `bank[i]` represents the <code>i<sup>th</sup></code> row, consisting of `'0'`s and `'1'`s. `'0'` means the cell is empty, while`'1'` means the cell has a security device.

There is **one** laser beam between any **two** security devices **if both** conditions are met:
* The two devices are located on two **different rows**: <code>r<sub>1</sub></code> and <code>r<sub>2</sub></code>, where <code>r<sub>1</sub> < r<sub>2</sub></code>.
* For **each** row `i` where <code>r<sub>1</sub> < i < r<sub>2</sub></code>, there are **no security devices** in the <code>i<sup>th</sup></code> row.

Laser beams are independent, i.e., one beam does not interfere nor join with another.

Return *the total number of laser beams in the bank*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/12/24/laser1.jpg)
<pre>
<strong>Input:</strong> bank = ["011001","000000","010100","001000"]
<strong>Output:</strong> 8
<strong>Explanation:</strong> Between each of the following device pairs, there is one beam. In total, there are 8 beams:
 * bank[0][1] -- bank[2][1]
 * bank[0][1] -- bank[2][3]
 * bank[0][2] -- bank[2][1]
 * bank[0][2] -- bank[2][3]
 * bank[0][5] -- bank[2][1]
 * bank[0][5] -- bank[2][3]
 * bank[2][1] -- bank[3][2]
 * bank[2][3] -- bank[3][2]
Note that there is no beam between any device on the 0th row with any on the 3rd row.
This is because the 2nd row contains security devices, which breaks the second condition.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/12/24/laser2.jpg)
<pre>
<strong>Input:</strong> bank = ["000","111","000"]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There does not exist two devices located on two different rows.
</pre>

#### Constraints:
* `m == bank.length`
* `n == bank[i].length`
* `1 <= m, n <= 500`
* `bank[i][j]` is either `'0'` or `'1'`.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def numberOfBeams(self, bank: List[str]) -> int:
        prev = 0
        ret = 0

        for row in bank:
            curr = row.count('1')
            ret += prev * curr
            if curr > 0:
                prev = curr

        return ret
```
