# 1523. 在区间范围内统计奇数数目
给你两个非负整数 `low` 和 `high` 。请你返回 `low` 和 `high` 之间（包括二者）奇数的数目。

#### 示例 1:
<pre>
<strong>输入:</strong> low = 3, high = 7
<strong>输出:</strong> 3
<strong>解释:</strong> 3 到 7 之间奇数数字为 [3,5,7] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> low = 8, high = 10
<strong>输出:</strong> 1
<strong>解释:</strong> 8 到 10 之间奇数数字为 [9] 。
</pre>

#### 提示:
* `0 <= low <= high <= 10^9`

## 题解 (Ruby)

### 1. 数学
```Ruby
# @param {Integer} low
# @param {Integer} high
# @return {Integer}
def count_odds(low, high)
    return (high - low + low % 2 + high % 2) / 2
end
```
