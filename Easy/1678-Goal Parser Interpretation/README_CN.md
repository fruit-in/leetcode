# 1678. 设计 Goal 解析器
请你设计一个可以解释字符串 `command` 的 **Goal 解析器** 。`command` 由 `"G"`、`"()"` 和/或 `"(al)"` 按某种顺序组成。Goal 解析器会将 `"G"` 解释为字符串 `"G"`、`"()"` 解释为字符串 `"o"` ，`"(al)"` 解释为字符串 `"al"` 。然后，按原顺序将经解释得到的字符串连接成一个字符串。

给你字符串 `command` ，返回 **Goal 解析器** 对 `command` 的解释结果。

#### 示例 1:
<pre>
<strong>输入:</strong> command = "G()(al)"
<strong>输出:</strong> "Goal"
<strong>解释:</strong> Goal 解析器解释命令的步骤如下所示：
G -> G
() -> o
(al) -> al
最后连接得到的结果是 "Goal"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> command = "G()()()()(al)"
<strong>输出:</strong> "Gooooal"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> command = "(al)G(al)()()G"
<strong>输出:</strong> "alGalooG"
</pre>

#### 提示:
* `1 <= command.length <= 100`
* `command` 由 `"G"`、`"()"` 和/或 `"(al)"` 按某种顺序组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn interpret(command: String) -> String {
        (command + " ")
            .as_bytes()
            .windows(2)
            .map(|w| match (w[0], w[1]) {
                (b'G', _) => "G",
                (b'(', b')') => "o",
                (b'(', b'a') => "al",
                _ => "",
            })
            .collect()
    }
}
```
