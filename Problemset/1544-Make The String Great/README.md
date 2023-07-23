# 1544. Make The String Great
Given a string `s` of lower and upper case English letters.

A good string is a string which doesn't have **two adjacent characters** `s[i]` and `s[i + 1]` where:
* `0 <= i <= s.length - 2`
* `s[i]` is a lower-case letter and `s[i + 1]` is the same letter but in upper-case or **vice-versa**.

To make the string good, you can choose **two adjacent** characters that make the string bad and remove them. You can keep doing this until the string becomes good.

Return *the string* after making it good. The answer is guaranteed to be unique under the given constraints.

**Notice** that an empty string is also good.

#### Example 1:
<pre>
<strong>Input:</strong> s = "leEeetcode"
<strong>Output:</strong> "leetcode"
<strong>Explanation:</strong> In the first step, either you choose i = 1 or i = 2, both will result "leEeetcode" to be reduced to "leetcode".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "abBAcC"
<strong>Output:</strong> ""
<strong>Explanation:</strong> We have many possible scenarios, and all lead to the same answer. For example:
"abBAcC" --> "aAcC" --> "cC" --> ""
"abBAcC" --> "abBA" --> "aA" --> ""
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "s"
<strong>Output:</strong> "s"
</pre>

#### Constraints:
* `1 <= s.length <= 100`
* `s` contains only lower and upper case English letters.

## Solutions (Ruby)

### 1. Stack
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
