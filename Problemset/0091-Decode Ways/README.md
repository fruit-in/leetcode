# 91. Decode Ways
A message containing letters from `A-Z` is being encoded to numbers using the following mapping:
```
'A' -> 1
'B' -> 2
...
'Z' -> 26
```

Given a **non-empty** string containing only digits, determine the total number of ways to decode it.

#### Example 1:
<pre>
<strong>Input:</strong> "12"
<strong>Output:</strong> 2
<strong>Explanation:</strong> It could be decoded as "AB" (1 2) or "L" (12).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "226"
<strong>Output:</strong> 3
<strong>Explanation:</strong> It could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).
</pre>

## Solutions (Python)

### 1. Dynamic Programming
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

## Solutions (Ruby)

### 1. Dynamic Programming
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
