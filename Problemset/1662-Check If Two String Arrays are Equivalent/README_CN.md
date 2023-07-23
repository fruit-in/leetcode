# 1662. 检查两个字符串数组是否相等
给你两个字符串数组 `word1` 和 `word2` 。如果两个数组表示的字符串相同，返回 `true` ；否则，返回 `false` 。

**数组表示的字符串** 是由数组中的所有元素 **按顺序** 连接形成的字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> word1 = ["ab", "c"], word2 = ["a", "bc"]
<strong>输出:</strong> true
<strong>解释:</strong>
word1 表示的字符串为 "ab" + "c" -> "abc"
word2 表示的字符串为 "a" + "bc" -> "abc"
两个字符串相同，返回 true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> word1 = ["a", "cb"], word2 = ["ab", "c"]
<strong>输出:</strong> false
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> word1  = ["abc", "d", "defg"], word2 = ["abcddefg"]
<strong>输出:</strong> true
</pre>

#### 提示:
* <code>1 <= word1.length, word2.length <= 10<sup>3</sup></code>
* <code>1 <= word1[i].length, word2[i].length <= 10<sup>3</sup></code>
* <code>1 <= sum(word1[i].length), sum(word2[i].length) <= 10<sup>3</sup></code>
* `word1[i]` 和 `word2[i]` 由小写字母组成

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {String[]} word1
# @param {String[]} word2
# @return {Boolean}
def array_strings_are_equal(word1, word2)
  word1.join == word2.join
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.concat() == word2.concat()
    }
}
```
