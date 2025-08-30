# 1541. 平衡括号字符串的最少插入次数
给你一个括号字符串 `s` ，它只包含字符 `'('` 和 `')'` 。一个括号字符串被称为平衡的当它满足：
* 任何左括号 `'('` 必须对应两个连续的右括号 `'))'` 。
* 左括号 `'('` 必须在对应的连续两个右括号 `'))'` 之前。

比方说 `"())"`， `"())(())))"` 和 `"(())())))"` 都是平衡的， `")()"`， `"()))"` 和 `"(()))"` 都是不平衡的。

你可以在任意位置插入字符 `'('` 和 `')'` 使字符串平衡。

请你返回让 `s` 平衡的最少插入次数。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "(()))"
<strong>输出:</strong> 1
<strong>解释:</strong> 第二个左括号有与之匹配的两个右括号，但是第一个左括号只有一个右括号。我们需要在字符串结尾额外增加一个 ')' 使字符串变成平衡字符串 "(())))" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "())"
<strong>输出:</strong> 0
<strong>解释:</strong> 字符串已经平衡了。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "))())("
<strong>输出:</strong> 3
<strong>解释:</strong> 添加 '(' 去匹配最开头的 '))' ，然后添加 '))' 去匹配最后一个 '(' 。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s = "(((((("
<strong>输出:</strong> 12
<strong>解释:</strong> 添加 12 个 ')' 得到平衡字符串。
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> s = ")))))))"
<strong>输出:</strong> 5
<strong>解释:</strong> 在字符串开头添加 4 个 '(' 并在结尾添加 1 个 ')' ，字符串变成平衡字符串 "(((())))))))" 。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` 只包含 `'('` 和 `')'` 。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut stack = vec![];
        let mut ret = 0;

        for c in s.chars() {
            if c == '(' && *stack.last().unwrap_or(&'(') == ')' {
                stack.pop();
                stack.push('(');
                ret += 1;
            } else if c == '(' {
                stack.push('(');
            } else if stack.is_empty() {
                stack.push(')');
                ret += 1;
            } else if *stack.last().unwrap() == '(' {
                stack.pop();
                stack.push(')');
            } else {
                stack.pop();
            }
        }

        if *stack.last().unwrap_or(&'(') == ')' {
            stack.pop();
            ret += 1;
        }
        ret += stack.len() as i32 * 2;

        ret
    }
}
```
