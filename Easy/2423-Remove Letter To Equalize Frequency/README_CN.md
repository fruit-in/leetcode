# 2423. 删除字符使频率相同
给你一个下标从 **0** 开始的字符串 `word` ，字符串只包含小写英文字母。你需要选择 一个 下标并 **删除** 下标处的字符，使得 `word` 中剩余每个字母出现 **频率** 相同。

如果删除一个字母后，`word` 中剩余所有字母的出现频率都相同，那么返回 `true` ，否则返回 `false` 。

**注意：**

* 字母 `x` 的 **频率** 是这个字母在字符串中出现的次数。
* 你 **必须** 恰好删除一个字母，不能一个字母都不删除。

#### 示例 1:
<pre>
<strong>输入:</strong> word = "abcc"
<strong>输出:</strong> true
<strong>解释:</strong> 选择下标 3 并删除该字母，word 变成 "abc" 且每个字母出现频率都为 1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> word = "aazz"
<strong>输出:</strong> false
<strong>解释:</strong> 我们必须删除一个字母，所以要么 "a" 的频率变为 1 且 "z" 的频率为 2 ，要么两个字母频率反过来。所以不可能让剩余所有字母出现频率相同。
</pre>

#### 提示:
* `2 <= word.length <= 100`
* `word` 只包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let word = word.as_bytes();

        for i in 0..word.len() {
            let mut count = [0; 26];

            for j in (0..i).chain(i + 1..word.len()) {
                count[(word[j] - b'a') as usize] += 1;
            }

            let x = *count.iter().find(|&&x| x > 0).unwrap();

            if count.iter().all(|&c| c == 0 || c == x) {
                return true;
            }
        }

        false
    }
}
```
