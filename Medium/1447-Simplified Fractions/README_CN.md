# 1447. 最简分数
给你一个整数 `n` ，请你返回所有 0 到 1 之间（不包括 0 和 1）满足分母小于等于  `n` 的 **最简** 分数 。分数可以以 **任意** 顺序返回。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 2
<strong>输出:</strong> ["1/2"]
<strong>解释:</strong> "1/2" 是唯一一个分母小于等于 2 的最简分数。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 3
<strong>输出:</strong> ["1/2","1/3","2/3"]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 4
<strong>输出:</strong> ["1/2","1/3","1/4","2/3","3/4"]
<strong>解释:</strong> "2/4" 不是最简分数，因为它可以化简为 "1/2" 。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> n = 1
<strong>输出:</strong> []
</pre>

#### 提示:
* `1 <= n <= 100`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def simplifiedFractions(self, n: int) -> List[str]:
        return [
            "{}/{}".format(j, i)
            for i in range(2, n + 1)
            for j in range(1, i)
            if gcd(i, j) == 1
        ]
```

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer} n
# @return {String[]}
def simplified_fractions(n)
  ret = []

  (2..n).each do |i|
    (1...i).each do |j|
      ret.push(format('%d/%d', j, i)) if i.gcd(j) == 1
    end
  end

  ret
end
```
