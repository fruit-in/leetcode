# 1415. 长度为 n 的开心字符串中字典序第 k 小的字符串
一个 「开心字符串」定义为：
* 仅包含小写字母 `['a', 'b', 'c']`.
* 对所有在 `1` 到 `s.length - 1` 之间的 `i` ，满足 `s[i] != s[i + 1]` （字符串的下标从 1 开始）。

比方说，字符串 **"abc"**，**"ac"**，**"b"** 和 **"abcbabcbcb"** 都是开心字符串，但是 **"aa"**，**"baa"** 和 **"ababbc"** 都不是开心字符串。

给你两个整数 `n` 和 `k` ，你需要将长度为 `n` 的所有开心字符串按字典序排序。

请你返回排序后的第 `k` 个开心字符串，如果长度为 `n` 的开心字符串少于 `k` 个，那么请你返回 **空字符串** 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 1, k = 3
<strong>输出:</strong> "c"
<strong>解释:</strong> 列表 ["a", "b", "c"] 包含了所有长度为 1 的开心字符串。按照字典序排序后第三个字符串为 "c" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 1, k = 4
<strong>输出:</strong> ""
<strong>解释:</strong> 长度为 1 的开心字符串只有 3 个。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 3, k = 9
<strong>输出:</strong> "cab"
<strong>解释:</strong> 长度为 3 的开心字符串总共有 12 个 ["aba", "abc", "aca", "acb", "bab", "bac", "bca", "bcb", "cab", "cac", "cba", "cbc"] 。第 9 个字符串为 "cab"
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> n = 2, k = 7
<strong>输出:</strong> ""
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> n = 10, k = 100
<strong>输出:</strong> "abacbabacb"
</pre>

#### 提示:
* `1 <= n <= 10`
* `1 <= k <= 100`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def getHappyString(self, n: int, k: int) -> str:
        happy = list("abc")

        for _ in range(n - 1):
            happy_ = []

            for s in happy:
                for c in "abc":
                    if s[-1] != c:
                        happy_.append(s + c)

            happy = happy_

        return "" if k > len(happy) else happy[k - 1]
```

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer} n
# @param {Integer} k
# @return {String}
def get_happy_string(n, k)
  happy = %w[a b c]

  (2..n).each do |_i|
    happy_ = []

    happy.each do |s|
      'abc'.chars.each do |c|
        happy_.push(s + c) if s[-1] != c
      end
    end

    happy = happy_
  end

  k > happy.size ? '' : happy[k - 1]
end
```
