# 1249. 移除无效的括号
给你一个由 `'('`、`')'` 和小写字母组成的字符串 `s`。

你需要从字符串中删除最少数目的 `'('` 或者 `')'` （可以删除任意位置的括号)，使得剩下的「括号字符串」有效。

请返回任意一个合法字符串。

有效「括号字符串」应当符合以下 **任意一条** 要求：
* 空字符串或只包含小写字母的字符串
* 可以被写作 `AB`（`A` 连接 `B`）的字符串，其中 `A` 和 `B` 都是有效「括号字符串」
* 可以被写作 `(A)` 的字符串，其中 `A` 是一个有效的「括号字符串」

#### 示例 1:
<pre>
<b>输入:</b> s = "lee(t(c)o)de)"
<b>输出:</b> "lee(t(c)o)de"
<b>解释:</b> "lee(t(co)de)" , "lee(t(c)ode)" 也是一个可行答案。
</pre>

#### 示例 2:
<pre>
<b>输入:</b> s = "a)b(c)d"
<b>输出:</b> "ab(c)d"
</pre>

#### 示例 3:
<pre>
<b>输入:</b> s = "))(("
<b>输出:</b> ""
<b>解释:</b> 空字符串也是有效的
</pre>

#### 示例 4:
<pre>
<b>输入:</b> s = "(a(b(c)d)"
<b>输出:</b> "a(b(c)d)"
</pre>

#### 提示:
* `1 <= s.length <= 10^5`
* `s[i]` 可能是 `'('`、`')'` 或英文小写字母

## 题解 (Rust)

### 1. 栈
```Rust
impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut lp = vec![];
        let mut ret = vec![];

        for ch in s.bytes() {
            if ch != b')' || lp.len() > 0 {
                ret.push(ch);
            }
            if ch == b'(' {
                lp.push(ret.len() - 1);
            } else if ch == b')' {
                lp.pop();
            }
        }

        while let Some(i) = lp.pop() {
            ret.remove(i);
        }

        String::from_utf8(ret).unwrap()
    }
}
```
