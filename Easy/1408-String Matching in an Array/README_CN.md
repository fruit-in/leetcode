# 1408. 数组中的字符串匹配
给你一个字符串数组 `words` ，数组中的每个字符串都可以看作是一个单词。请你按 **任意** 顺序返回 `words` 中是其他单词的子字符串的所有单词。

如果你可以删除 `words[j]` 最左侧和/或最右侧的若干字符得到 `word[i]` ，那么字符串 `words[i]` 就是 `words[j]` 的一个子字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> words = ["mass","as","hero","superhero"]
<strong>输出:</strong> ["as","hero"]
<strong>解释:</strong> "as" 是 "mass" 的子字符串，"hero" 是 "superhero" 的子字符串。
["hero","as"] 也是有效的答案。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> words = ["leetcode","et","code"]
<strong>输出:</strong> ["et","code"]
<strong>解释:</strong> "et" 和 "code" 都是 "leetcode" 的子字符串。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> words = ["blue","green","bu"]
<strong>输出:</strong> []
</pre>

#### 提示:
* `1 <= words.length <= 100`
* `1 <= words[i].length <= 30`
* `words[i]` 仅包含小写英文字母。
* 题目数据 **保证** 每个 `words[i]` 都是独一无二的。

## 题解 (Rust)

### 1. 暴力
```Rust
impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut ret = Vec::new();

        for i in 0..words.len() {
            for j in 0..words.len() {
                if j != i && words[j].contains(&words[i]) {
                    ret.push(words[i].clone());
                    break;
                }
            }
        }

        ret
    }
}
```
