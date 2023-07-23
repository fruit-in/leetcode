# 647. Palindromic Substrings
Given a string, your task is to count how many palindromic substrings in this string.

The substrings with different start indexes or end indexes are counted as different substrings even they consist of same characters.

#### Example 1:
<pre>
<strong>Input:</strong> "abc"
<strong>Output:</strong> 3
<strong>Explanation:</strong> Three palindromic strings: "a", "b", "c".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "aaa"
<strong>Output:</strong> 6
<strong>Explanation:</strong> Six palindromic strings: "a", "a", "a", "aa", "aa", "aaa".
</pre>

#### Note:
1. The input string length won't exceed 1000.

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {String} s
# @return {Integer}
def count_substrings(s)
  ret = 0

  (0...s.size).each do |i|
    (0..[i, s.size - 1 - i].min).each do |j|
      if s[i - j] == s[i + j]
        ret += 1
      else
        break
      end
    end
    next unless i < s.size - 1 && s[i] == s[i + 1]

    (0..[i, s.size - 2 - i].min).each do |j|
      if s[i - j] == s[i + 1 + j]
        ret += 1
      else
        break
      end
    end
  end

  ret
end
```
