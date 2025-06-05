# 1478. 安排邮筒
给你一个房屋数组`houses` 和一个整数 `k` ，其中 `houses[i]` 是第 `i` 栋房子在一条街上的位置，现需要在这条街上安排 `k` 个邮筒。

请你返回每栋房子与离它最近的邮筒之间的距离的 **最小** 总和。

答案保证在 32 位有符号整数范围以内。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/05/07/sample_11_1816.png)
<pre>
<strong>输入:</strong> houses = [1,4,8,10,20], k = 3
<strong>输出:</strong> 5
<strong>解释:</strong> 将邮筒分别安放在位置 3， 9 和 20 处。
每个房子到最近邮筒的距离和为 |3-1| + |4-3| + |9-8| + |10-9| + |20-20| = 5 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/05/07/sample_2_1816.png)
<pre>
<strong>输入:</strong> houses = [2,3,5,12,18], k = 2
<strong>输出:</strong> 9
<strong>解释:</strong> 将邮筒分别安放在位置 3 和 14 处。
每个房子到最近邮筒距离和为 |2-3| + |3-3| + |5-3| + |12-14| + |18-14| = 9 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> houses = [7,4,6,1], k = 1
<strong>输出:</strong> 8
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> houses = [3,6,14,10], k = 4
<strong>输出:</strong> 0
</pre>

#### 提示:
* `n == houses.length`
* `1 <= n <= 100`
* <code>1 <= houses[i] <= 10<sup>4</sup></code>
* `1 <= k <= n`
* 数组 `houses` 中的整数互不相同。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def minDistance(self, houses: List[int], k: int) -> int:
        @cache
        def dfs(i: int, k: int) -> int:
            if i == len(houses):
                return 0
            if k == 0:
                return inf

            ret = inf

            for j in range(i + 1, len(houses) + 1):
                mid = (i + j) // 2
                distl = houses[mid] * (mid - i) - prefixsum[mid] + prefixsum[i]
                distr = prefixsum[j] - prefixsum[mid] - houses[mid] * (j - mid)
                ret = min(ret, distl + distr + dfs(j, k - 1))

            return ret

        prefixsum = [0] * (len(houses) + 1)
        houses.sort()

        for i in range(len(houses)):
            prefixsum[i + 1] = prefixsum[i] + houses[i]

        return dfs(0, k)
```
