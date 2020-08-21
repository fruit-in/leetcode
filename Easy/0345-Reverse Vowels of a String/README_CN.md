# 345. 反转字符串中的元音字母
编写一个函数，以字符串作为输入，反转该字符串中的元音字母。

#### 示例 1:
<pre>
<strong>输入:</strong> "hello"
<strong>输出:</strong> "holle"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "leetcode"
<strong>输出:</strong> "leotcede"
</pre>

#### 说明:
元音字母不包含字母"y"。

## 题解 (Python)

### 1. 双指针
```Python3
class Solution:
    def reverseVowels(self, s: str) -> str:
        s = list(s)
        vowels = "aiueoAIUEO"
        i = 0
        j = len(s) - 1
        while i < j:
            while i < j and s[i] not in vowels:
                i += 1
            while i < j and s[j] not in vowels:
                j -= 1
            s[i], s[j] = s[j], s[i]
            i += 1
            j -= 1
        return ''.join(s)
```

## 题解 (Ruby)

### 1. 双指针
```Ruby
# @param {String} s
# @return {String}
def reverse_vowels(s)
    vowels = "aiueoAIUEO"
    i, j = 0, s.length - 1

    while i < j
        while i < j and not vowels.include?(s[i])
            i += 1
        end
        while i < j and not vowels.include?(s[j])
            j -= 1
        end

        s[i], s[j] = s[j], s[i]

        i += 1
        j -= 1
    end

    return s
end
```
