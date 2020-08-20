# 1456. Maximum Number of Vowels in a Substring of Given Length
Given a string `s` and an integer `k`.

Return *the maximum number of vowel letters* in any substring of `s` with length `k`.

**Vowel letters** in English are (a, e, i, o, u).

#### Example 1:
<pre>
<strong>Input:</strong> s = "abciiidef", k = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> The substring "iii" contains 3 vowel letters.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "aeiou", k = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> Any substring of length 2 contains 2 vowels.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "leetcode", k = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> "lee", "eet" and "ode" contain 2 vowels.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> s = "rhythms", k = 4
<strong>Output:</strong> 0
<strong>Explanation:</strong> We can see that s doesn't have any vowel letters.
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> s = "tryhard", k = 4
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `1 <= s.length <= 10^5`
* `s` consists of lowercase English letters.
* `1 <= k <= s.length`

## Solutions (Python)

### 1. Sliding Window
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

## Solutions (Ruby)

### 1. Sliding Window
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
