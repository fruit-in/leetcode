# 2125. 银行中的激光束数量
银行内部的防盗安全装置已经激活。给你一个下标从 **0** 开始的二进制字符串数组 `bank` ，表示银行的平面图，这是一个大小为 `m x n` 的二维矩阵。 `bank[i]` 表示第 `i` 行的设备分布，由若干 `'0'` 和若干 `'1'` 组成。`'0'` 表示单元格是空的，而 `'1'` 表示单元格有一个安全设备。

对任意两个安全设备而言，**如果同时** 满足下面两个条件，则二者之间存在 一个 激光束：
* 两个设备位于两个 **不同行** ：<code>r<sub>1</sub></code> 和 <code>r<sub>2</sub></code> ，其中 <code>r<sub>1</sub> < r<sub>2</sub></code> 。
* 满足 <code>r<sub>1</sub> < i < r<sub>2</sub></code> 的 所有 行 `i` ，都 **没有安全设备** 。

激光束是独立的，也就是说，一个激光束既不会干扰另一个激光束，也不会与另一个激光束合并成一束。

返回银行中激光束的总数量。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/12/24/laser1.jpg)
<pre>
<strong>输入:</strong> bank = ["011001","000000","010100","001000"]
<strong>输出:</strong> 8
<strong>解释:</strong> 在下面每组设备对之间，存在一条激光束。总共是 8 条激光束：
 * bank[0][1] -- bank[2][1]
 * bank[0][1] -- bank[2][3]
 * bank[0][2] -- bank[2][1]
 * bank[0][2] -- bank[2][3]
 * bank[0][5] -- bank[2][1]
 * bank[0][5] -- bank[2][3]
 * bank[2][1] -- bank[3][2]
 * bank[2][3] -- bank[3][2]
注意，第 0 行和第 3 行上的设备之间不存在激光束。
这是因为第 2 行存在安全设备，这不满足第 2 个条件。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/12/24/laser2.jpg)
<pre>
<strong>输入:</strong> bank = ["000","111","000"]
<strong>输出:</strong> 0
<strong>解释:</strong> 不存在两个位于不同行的设备
</pre>

#### 提示:
* `m == bank.length`
* `n == bank[i].length`
* `1 <= m, n <= 500`
* `bank[i][j]` 为 `'0'` 或 `'1'`

## 题解 (Python)

### 1. 题解
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
