# 1544. 整理字符串
给你一个由大小写英文字母组成的字符串 `s` 。

一个整理好的字符串中，**两个相邻字符** `s[i]` 和 `s[i + 1]` 不会同时满足下述条件：
* `0 <= i <= s.length - 2`
* `s[i]` 是小写字符，但 `s[i + 1]` 是相同的大写字符；**反之亦然** 。

请你将字符串整理好，每次你都可以从字符串中选出满足上述条件的 **两个相邻** 字符并删除，直到字符串整理好为止。

请返回整理好的 **字符串** 。题目保证在给出的约束条件下，测试样例对应的答案是唯一的。

**注意:** 空字符串也属于整理好的字符串，尽管其中没有任何字符。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "leEeetcode"
<strong>输出:</strong> "leetcode"
<strong>解释:</strong> 无论你第一次选的是 i = 1 还是 i = 2，都会使 "leEeetcode" 缩减为 "leetcode" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "abBAcC"
<strong>输出:</strong> ""
<strong>解释:</strong> 存在多种不同情况，但所有的情况都会导致相同的结果。例如：
"abBAcC" --> "aAcC" --> "cC" --> ""
"abBAcC" --> "abBA" --> "aA" --> ""
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "s"
<strong>输出:</strong> "s"
</pre>

#### 提示:
* `1 <= s.length <= 100`
* `s` 只包含小写和大写英文字母

## 题解 (Ruby)

### 1. 栈
```Ruby
# @param {String} s
# @return {String}
def make_good(s)
    stack = []

    for ch in s.chars
        if not stack.empty? and (stack[-1].ord - ch.ord).abs == 32
            stack.pop
        else
            stack.push(ch)
        end
    end

    return stack.join
end
```
