# 2068. 检查两个字符串是否几乎相等
如果两个字符串 `word1` 和 `word2` 中从 `'a'` 到 `'z'` 每一个字母出现频率之差都 **不超过** `3` ，那么我们称这两个字符串 `word1` 和 `word2` **几乎相等** 。

给你两个长度都为 `n` 的字符串 `word1` 和 `word2` ，如果 `word1` 和 `word2` **几乎相等** ，请你返回 `true` ，否则返回 `false` 。

一个字母 `x` 的出现 **频率** 指的是它在字符串中出现的次数。

#### 示例 1:
<pre>
<strong>输入:</strong> word1 = "aaaa", word2 = "bccb"
<strong>输出:</strong> false
<strong>解释:</strong> 字符串 "aaaa" 中有 4 个 'a' ，但是 "bccb" 中有 0 个 'a' 。
两者之差为 4 ，大于上限 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> word1 = "abcdeef", word2 = "abaaacc"
<strong>输出:</strong> true
<strong>解释:</strong> word1 和 word2 中每个字母出现频率之差至多为 3 ：
- 'a' 在 word1 中出现了 1 次，在 word2 中出现了 4 次，差为 3 。
- 'b' 在 word1 中出现了 1 次，在 word2 中出现了 1 次，差为 0 。
- 'c' 在 word1 中出现了 1 次，在 word2 中出现了 2 次，差为 1 。
- 'd' 在 word1 中出现了 1 次，在 word2 中出现了 0 次，差为 1 。
- 'e' 在 word1 中出现了 2 次，在 word2 中出现了 0 次，差为 2 。
- 'f' 在 word1 中出现了 1 次，在 word2 中出现了 0 次，差为 1 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> word1 = "cccddabba", word2 = "babababab"
<strong>输出:</strong> true
<strong>解释:</strong> word1 和 word2 中每个字母出现频率之差至多为 3 ：
- 'a' 在 word1 中出现了 2 次，在 word2 中出现了 4 次，差为 2 。
- 'b' 在 word1 中出现了 2 次，在 word2 中出现了 5 次，差为 3 。
- 'c' 在 word1 中出现了 3 次，在 word2 中出现了 0 次，差为 3 。
- 'd' 在 word1 中出现了 2 次，在 word2 中出现了 0 次，差为 2 。
</pre>

#### 提示:
* `n == word1.length == word2.length`
* `1 <= n <= 100`
* `word1` 和 `word2` 都只包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let mut count = [0_i32; 26];

        for i in 0..word1.len() {
            count[(word1[i] - b'a') as usize] += 1;
            count[(word2[i] - b'a') as usize] -= 1;
        }

        count.iter().all(|&x| x.abs() <= 3)
    }
}
```
