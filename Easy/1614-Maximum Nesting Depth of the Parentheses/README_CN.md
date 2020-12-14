# 1614. 括号的最大嵌套深度
如果字符串满足以下条件之一，则可以称之为 **有效括号字符串**（**valid parentheses string**，可以简写为 **VPS**）：
* 字符串是一个空字符串 `""`，或者是一个不为 `"("` 或 `")"` 的单字符。
* 字符串可以写为 `AB`（`A` 与 `B` 字符串连接），其中 `A` 和 `B` 都是 **有效括号字符串** 。
* 字符串可以写为 `(A)`，其中 `A` 是一个 **有效括号字符串** 。

类似地，可以定义任何有效括号字符串 `S` 的 **嵌套深度** `depth(S)`：
* `depth("") = 0`
* `depth(C) = 0`，其中 `C` 是单个字符的字符串，且该字符不是 `"("` 或者 `")"`
* `depth(A + B) = max(depth(A), depth(B))`，其中 `A` 和 `B` 都是 **有效括号字符串**
* `depth("(" + A + ")") = 1 + depth(A)`，其中 `A` 是一个 **有效括号字符串**

例如：`""`、`"()()"`、`"()(()())"` 都是 **有效括号字符串**（嵌套深度分别为 0、1、2），而 `")("` 、`"(()"` 都不是 **有效括号字符串** 。

给你一个 **有效括号字符串** `s`，返回该字符串的 `s` **嵌套深度** 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "(1+(2*3)+((8)/4))+1"
<strong>输出:</strong> 3
<strong>解释:</strong> 数字 8 在嵌套的 3 层括号中。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "(1)+((2))+(((3)))"
<strong>输出:</strong> 3
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "1+(2*3)/(2-1)"
<strong>输出:</strong> 1
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s = "1"
<strong>输出:</strong> 0
</pre>

#### 提示:
* `1 <= s.length <= 100`
* `s` 由数字 `0-9` 和字符 `'+'`、`'-'`、`'*'`、`'/'`、`'('`、`')'` 组成
* 题目数据保证括号表达式 `s` 是 **有效的括号表达式**

## 题解 (Ruby)

### 1. 计数
```Ruby
# @param {String} s
# @return {Integer}
def max_depth(s)
  left_count = 0
  ret = 0

  s.chars.each do |ch|
    if ch == '('
      left_count += 1
      ret = left_count if left_count > ret
    elsif ch == ')'
      left_count -= 1
    end
  end

  ret
end
```

## 题解 (Rust)

### 1. 计数
```Rust
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut left_count = 0;
        let mut ret = 0;

        for ch in s.chars() {
            match ch {
                '(' => {
                    left_count += 1;
                    ret = ret.max(left_count);
                }
                ')' => left_count -= 1,
                _ => (),
            }
        }

        ret
    }
}
```
