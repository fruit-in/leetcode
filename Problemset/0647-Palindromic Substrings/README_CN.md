# 647. 回文子串
给定一个字符串，你的任务是计算这个字符串中有多少个回文子串。

具有不同开始位置或结束位置的子串，即使是由相同的字符组成，也会被视作不同的子串。

#### 示例 1:
<pre>
<strong>输入:</strong> "abc"
<strong>输出:</strong> 3
<strong>解释:</strong> 三个回文子串: "a", "b", "c"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "aaa"
<strong>输出:</strong> 6
<strong>解释:</strong> 6个回文子串: "a", "a", "a", "aa", "aa", "aaa"
</pre>

#### 提示:
* 输入的字符串长度不会超过 1000 。

## 题解 (Ruby)

### 1. 题解
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
