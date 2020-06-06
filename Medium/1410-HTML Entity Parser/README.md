# 1410. HTML Entity Parser
**HTML entity parser** is the parser that takes HTML code as input and replace all the entities of the special characters by the characters itself.

The special characters and their entities for HTML are:
* **Quotation Mark:** the entity is `&quot;` and symbol character is `"`.
* **Single Quote Mark:** the entity is `&apos;` and symbol character is `'`.
* **Ampersand:** the entity is `&amp;` and symbol character is `&`.
* **Greater Than Sign:** the entity is `&gt;` and symbol character is `>`.
* **Less Than Sign:** the entity is `&lt;` and symbol character is `<`.
* **Slash:** the entity is `&frasl;` and symbol character is `/`.

Given the input `text` string to the HTML parser, you have to implement the entity parser.

Return *the text* after replacing the entities by the special characters.

#### Example 1:
<pre>
<strong>Input:</strong> text = "&amp;amp; is an HTML entity but &ambassador; is not."
<strong>Output:</strong> "& is an HTML entity but &ambassador; is not."
<strong>Explanation:</strong> The parser will replace the &amp;amp; entity by &
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> text = "and I quote: &amp;quot;...&amp;quot;"
<strong>Output:</strong> "and I quote: \"...\""
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> text = "Stay home! Practice on Leetcode :)"
<strong>Output:</strong> "Stay home! Practice on Leetcode :)"
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> text = "x &amp;gt; y &amp;amp;&amp;amp; x &amp;lt; y is always false"
<strong>Output:</strong> "x > y && x < y is always false"
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> text = "leetcode.com&amp;frasl;problemset&amp;frasl;all"
<strong>Output:</strong> "leetcode.com/problemset/all"
</pre>

#### Constraints:
* `1 <= text.length <= 10^5`
* The string may contain any possible characters out of all the 256 ASCII characters.

## Solutions (Rust)

### 1. Replace
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
