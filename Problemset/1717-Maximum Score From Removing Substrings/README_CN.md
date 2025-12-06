# 1717. 删除子字符串的最大得分
给你一个字符串 `s` 和两个整数 `x` 和 `y` 。你可以执行下面两种操作任意次。

* 删除子字符串 `"ab"` 并得到 `x` 分。
    * 比方说，从 `"cabxbae"` 删除 `ab` ，得到 `"cxbae"` 。
* 删除子字符串 `"ba"` 并得到 `y` 分。
    * 比方说，从 `"cabxbae"` 删除 `ba` ，得到 `"cabxe"` 。

请返回对 `s` 字符串执行上面操作若干次能得到的最大得分。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "cdbcbbaaabab", x = 4, y = 5
<strong>输出:</strong> 19
<strong>解释:</strong>
- 删除 "cdbcbbaaabab" 中加粗的 "ba" ，得到 s = "cdbcbbaaab" ，加 5 分。
- 删除 "cdbcbbaaab" 中加粗的 "ab" ，得到 s = "cdbcbbaa" ，加 4 分。
- 删除 "cdbcbbaa" 中加粗的 "ba" ，得到 s = "cdbcba" ，加 5 分。
- 删除 "cdbcba" 中加粗的 "ba" ，得到 s = "cdbc" ，加 5 分。
总得分为 5 + 4 + 5 + 5 = 19 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "aabbaaxybbaabb", x = 5, y = 4
<strong>输出:</strong> 20
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* <code>1 <= x, y <= 10<sup>4</sup></code>
* `s` 只包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        if x < y {
            return Self::maximum_gain(
                s.replace("a", "$").replace("b", "a").replace("$", "b"),
                y,
                x,
            );
        }

        let mut stack1 = vec![];
        let mut ret = 0;

        for c in s.chars().chain(std::iter::once('c')) {
            if c == 'b' && stack1.last() == Some(&'a') {
                stack1.pop();
                ret += x;
            } else if c == 'a' || c == 'b' {
                stack1.push(c);
            } else {
                let mut stack2 = vec![];

                for c in stack1.drain(..) {
                    if c == 'a' && stack2.last() == Some(&'b') {
                        stack2.pop();
                        ret += y;
                    } else {
                        stack2.push(c);
                    }
                }
            }
        }

        ret
    }
}
```
