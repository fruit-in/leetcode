# 1410. HTML 实体解析器
「HTML 实体解析器」 是一种特殊的解析器，它将 HTML 代码作为输入，并用字符本身替换掉所有这些特殊的字符实体。

HTML 里这些特殊字符和它们对应的字符实体包括：
* **双引号：** 字符实体为 `&quot;` ，对应的字符是 `"` 。
* **单引号：** 字符实体为 `&apos;` ，对应的字符是 `'` 。
* **与符号：** 字符实体为 `&amp;` ，对应对的字符是 `&` 。
* **大于号：** 字符实体为 `&gt;` ，对应的字符是 `>` 。
* **小于号：** 字符实体为 `&lt;` ，对应的字符是 `<` 。
* **斜线号：** 字符实体为 `&frasl;` ，对应的字符是 `/` 。

给你输入字符串 `text` ，请你实现一个 HTML 实体解析器，返回解析器解析后的结果。

#### 示例 1:
<pre>
<strong>输入:</strong> text = "&amp;amp; is an HTML entity but &ambassador; is not."
<strong>输出:</strong> "& is an HTML entity but &ambassador; is not."
<strong>解释:</strong> 解析器把字符实体 &amp;amp; 用 & 替换
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> text = "and I quote: &amp;quot;...&amp;quot;"
<strong>输出:</strong> "and I quote: \"...\""
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> text = "Stay home! Practice on Leetcode :)"
<strong>输出:</strong> "Stay home! Practice on Leetcode :)"
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> text = "x &amp;gt; y &amp;amp;&amp;amp; x &amp;lt; y is always false"
<strong>输出:</strong> "x > y && x < y is always false"
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> text = "leetcode.com&amp;frasl;problemset&amp;frasl;all"
<strong>输出:</strong> "leetcode.com/problemset/all"
</pre>

#### 提示:
* `1 <= text.length <= 10^5`
* 字符串可能包含 256 个ASCII 字符中的任意字符。

## 题解 (Rust)

### 1. 字符串替换
```Rust
impl Solution {
    pub fn entity_parser(text: String) -> String {
        text.replace("&quot;", "\"")
            .replace("&apos;", "'")
            .replace("&gt;", ">")
            .replace("&lt;", "<")
            .replace("&frasl;", "/")
            .replace("&amp;", "&")
    }
}
```
