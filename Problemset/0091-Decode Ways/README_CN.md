# 91. 解码方法
一条包含字母 `A-Z` 的消息通过以下方式进行了编码：
```
'A' -> 1
'B' -> 2
...
'Z' -> 26
```

给定一个只包含数字的**非空**字符串，请计算解码方法的总数。

#### 示例 1:
<pre>
<strong>输入:</strong> "12"
<strong>输出:</strong> 2
<strong>解释:</strong> 它可以解码为 "AB"（1 2）或者 "L"（12）。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "226"
<strong>输出:</strong> 3
<strong>解释:</strong> 它可以解码为 "BZ" (2 26), "VF" (22 6), 或者 "BBF" (2 2 6) 。
</pre>

## 题解 (Python)

### 1. 动态规划
```Python
class Solution:
    def numDecodings(self, s: str) -> int:
        if s[0] == '0':
            return 0

        prev, curr = 1, 1

        for i in range(1, len(s)):
            if s[i] == '0' and (s[i - 1] > '2' or s[i - 1] == '0'):
                return 0
            elif s[i] == '0':
                curr = prev
            elif "10" < s[i - 1:i + 1] < "27":
                prev, curr = curr, prev + curr
            else:
                prev = curr

        return curr
```

## 题解 (Ruby)

### 1. 动态规划
```Ruby
# @param {String} s
# @return {Integer}
def num_decodings(s)
    return 0 if s[0] == '0'

    prev, curr = 1, 1

    for i in 1...s.length
        if s[i] == '0' and (s[i - 1] > '2' or s[i - 1] == '0')
            return 0
        elsif s[i] == '0'
            curr = prev
        elsif s[i - 1..i] > "10" and s[i - 1..i] < "27"
            prev, curr = curr, prev + curr
        else
            prev = curr
        end
    end

    return curr
end
```
