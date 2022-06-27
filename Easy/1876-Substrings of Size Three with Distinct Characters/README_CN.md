# 1876. 长度为三且各字符不同的子字符串
如果一个字符串不含有任何重复字符，我们称这个字符串为 **好** 字符串。

给你一个字符串 `s` ，请你返回 `s` 中长度为 **3** 的 **好子字符串** 的数量。

注意，如果相同的好子字符串出现多次，每一次都应该被记入答案之中。

**子字符串** 是一个字符串中连续的字符序列。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "xyzzaz"
<strong>输出:</strong> 1
<strong>解释:</strong> 总共有 4 个长度为 3 的子字符串："xyz"，"yzz"，"zza" 和 "zaz" 。
唯一的长度为 3 的好子字符串是 "xyz" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "aababcabc"
<strong>输出:</strong> 4
<strong>解释:</strong> 总共有 7 个长度为 3 的子字符串："aab"，"aba"，"bab"，"abc"，"bca"，"cab" 和 "abc" 。
好子字符串包括 "abc"，"bca"，"cab" 和 "abc" 。
</pre>

#### 提示:
* `1 <= s.length <= 100`
* `s` 只包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        s.into_bytes()
            .windows(3)
            .filter(|gs| gs[0] != gs[1] && gs[0] != gs[2] && gs[1] != gs[2])
            .count() as i32
    }
}
```
