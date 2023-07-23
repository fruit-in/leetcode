# 2185. 统计包含给定前缀的字符串
给你一个字符串数组 `words` 和一个字符串 `pref` 。

返回 `words` 中以 `pref` 作为 **前缀** 的字符串的数目。

字符串 `s` 的 **前缀** 就是  `s` 的任一前导连续字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> words = ["pay","attention","practice","attend"], pref = "at"
<strong>输出:</strong> 2
<strong>解释:</strong> 以 "at" 作为前缀的字符串有两个，分别是："attention" 和 "attend" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> words = ["leetcode","win","loops","success"], pref = "code"
<strong>输出:</strong> 0
<strong>解释:</strong> 不存在以 "code" 作为前缀的字符串。
</pre>

#### 提示:
* `1 <= words.length <= 100`
* `1 <= words[i].length, pref.length <= 100`
* `words[i]` 和 `pref` 由小写英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.iter().filter(|w| w.starts_with(&pref)).count() as i32
    }
}
```
