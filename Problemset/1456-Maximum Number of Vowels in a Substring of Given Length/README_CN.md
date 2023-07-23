# 1456. 定长子串中元音的最大数目
给你字符串 `s` 和整数 `k` 。

请返回字符串 `s` 中长度为 `k` 的单个子字符串中可能包含的最大元音字母数。

英文中的 **元音字母** 为（`a`, `e`, `i`, `o`, `u`）。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "abciiidef", k = 3
<strong>输出:</strong> 3
<strong>解释:</strong> 子字符串 "iii" 包含 3 个元音字母。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "aeiou", k = 2
<strong>输出:</strong> 2
<strong>解释:</strong> 任意长度为 2 的子字符串都包含 2 个元音字母。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "leetcode", k = 3
<strong>输出:</strong> 2
<strong>解释:</strong> "lee"、"eet" 和 "ode" 都包含 2 个元音字母。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s = "rhythms", k = 4
<strong>输出:</strong> 0
<strong>解释:</strong> 字符串 s 中不含任何元音字母。
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> s = "tryhard", k = 4
<strong>输出:</strong> 1
</pre>

#### 提示:
* `1 <= s.length <= 10^5`
* `s` 由小写英文字母组成
* `1 <= k <= s.length`

## 题解 (Python)

### 1. 滑动窗口
```Python
class Solution:
    def maxVowels(self, s: str, k: int) -> int:
        i = 0
        cnt = 0
        ret = 0

        for j in range(len(s)):
            if s[j] in "aeiou":
                cnt += 1
            if j - i == k:
                if s[i] in "aeiou":
                    cnt -= 1
                i += 1
            ret = max(ret, cnt)

        return ret
```

## 题解 (Ruby)

### 1. 滑动窗口
```Ruby
# @param {String} s
# @param {Integer} k
# @return {Integer}
def max_vowels(s, k)
    i = 0
    cnt = 0
    ret = 0

    for j in 0...s.length
        if "aeiou".include?(s[j])
            cnt += 1
        end
        if j - i == k
            if "aeiou".include?(s[i])
                cnt -= 1
            end
            i += 1
        end
        ret = [ret, cnt].max
    end

    return ret
end
```
