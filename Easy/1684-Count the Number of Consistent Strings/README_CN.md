# 1684. 统计一致字符串的数目
给你一个由不同字符组成的字符串 `allowed` 和一个字符串数组 `words` 。如果一个字符串的每一个字符都在 `allowed` 中，就称这个字符串是 **一致字符串** 。

请你返回 `words` 数组中 **一致字符串** 的数目。

#### 示例 1:
<pre>
<strong>输入:</strong> allowed = "ab", words = ["ad","bd","aaab","baa","badab"]
<strong>输出:</strong> 2
<strong>解释:</strong> 字符串 "aaab" 和 "baa" 都是一致字符串，因为它们只包含字符 'a' 和 'b' 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> allowed = "abc", words = ["a","b","c","ab","ac","bc","abc"]
<strong>输出:</strong> 7
<strong>解释:</strong> 所有字符串都是一致的。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> allowed = "cad", words = ["cc","acd","b","ba","bac","bad","ac","d"]
<strong>输出:</strong> 4
<strong>解释:</strong> 字符串 "cc"，"acd"，"ac" 和 "d" 是一致字符串。
</pre>

#### 提示:
* <code>1 <= words.length <= 10<sup>4</sup></code>
* `1 <= allowed.length <= 26`
* `1 <= words[i].length <= 10`
* `allowed` 中的字符 **互不相同** 。
* `words[i]` 和 `allowed` 只包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let allowed = allowed.bytes().fold(0, |ac, c| ac | (1 << (c - b'a')));

        words
            .iter()
            .filter(|word| word.bytes().all(|c| allowed & (1 << (c - b'a')) != 0))
            .count() as i32
    }
}
```
