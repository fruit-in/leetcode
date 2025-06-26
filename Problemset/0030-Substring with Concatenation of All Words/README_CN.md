# 30. 串联所有单词的子串
给定一个字符串 `s` 和一个字符串数组 `words`。 `words` 中所有字符串 **长度相同**。

`s` 中的 **串联子串** 是指一个包含  `words` 中所有字符串以任意顺序排列连接起来的子串。

* 例如，如果 `words = ["ab","cd","ef"]`， 那么 `"abcdef"`， `"abefcd"`，`"cdabef"`， `"cdefab"`，`"efabcd"`， 和 `"efcdab"` 都是串联子串。 `"acdbef"` 不是串联子串，因为他不是任何 `words` 排列的连接。

返回所有串联子串在 `s` 中的开始索引。你可以以 **任意顺序** 返回答案。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "barfoothefoobarman", words = ["foo","bar"]
<strong>输出:</strong> [0,9]
<strong>解释:</strong> 因为 words.length == 2 同时 words[i].length == 3，连接的子字符串的长度必须为 6。
子串 "barfoo" 开始位置是 0。它是 words 中以 ["bar","foo"] 顺序排列的连接。
子串 "foobar" 开始位置是 9。它是 words 中以 ["foo","bar"] 顺序排列的连接。
输出顺序无关紧要。返回 [9,0] 也是可以的。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]
<strong>输出:</strong> []
<strong>解释:</strong> 因为 words.length == 4 并且 words[i].length == 4，所以串联子串的长度必须为 16。
s 中没有子串长度为 16 并且等于 words 的任何顺序排列的连接。
所以我们返回一个空数组。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "barfoofoobarthefoobarman", words = ["bar","foo","the"]
<strong>输出:</strong> [6,9,12]
<strong>解释:</strong> 因为 words.length == 3 并且 words[i].length == 3，所以串联子串的长度必须为 9。
子串 "foobarthe" 开始位置是 6。它是 words 中以 ["foo","bar","the"] 顺序排列的连接。
子串 "barthefoo" 开始位置是 9。它是 words 中以 ["bar","the","foo"] 顺序排列的连接。
子串 "thefoobar" 开始位置是 12。它是 words 中以 ["the","foo","bar"] 顺序排列的连接。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>4</sup></code>
* `1 <= words.length <= 5000`
* `1 <= words[i].length <= 30`
* `words[i]` 和 `s` 由小写英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if s.len() < words.len() * words[0].len() {
            return vec![];
        }

        const BASE1: i64 = 131;
        const BASE2: i64 = 257;
        const MOD1: i64 = 1_000_000_007;
        const MOD2: i64 = 1_000_000_009;
        let s = s.as_bytes();
        let m = s.len();
        let n = words[0].len();
        let concat_len = words.len() * n;
        let mut hash1 = 0;
        let mut hash2 = 0;
        let mut base_pow1 = (0..n).fold(1, |acc, _| acc * BASE1 % MOD1);
        let mut base_pow2 = (0..n).fold(1, |acc, _| acc * BASE2 % MOD2);
        let mut s_sub_hash = vec![(0, 0); m - n + 1];
        let mut words_hash_count = HashMap::new();
        let mut ret = vec![];

        for i in 0..m {
            hash1 = (hash1 * BASE1 + s[i] as i64) % MOD1;
            hash2 = (hash2 * BASE2 + s[i] as i64) % MOD2;

            if i >= n {
                hash1 = (hash1 - s[i - n] as i64 * base_pow1).rem_euclid(MOD1);
                hash2 = (hash2 - s[i - n] as i64 * base_pow2).rem_euclid(MOD2);
            }
            if i >= n - 1 {
                s_sub_hash[i + 1 - n] = (hash1, hash2);
            }
        }

        for word in &words {
            hash1 = 0;
            hash2 = 0;

            for c in word.bytes() {
                hash1 = (hash1 * BASE1 + c as i64) % MOD1;
                hash2 = (hash2 * BASE2 + c as i64) % MOD2;
            }

            *words_hash_count.entry((hash1, hash2)).or_insert(0) += 1;
        }

        for i in 0..n {
            let mut s_sub_hash_count = HashMap::new();
            let mut distinct_count = 0;
            let mut hash = (0, 0);

            for j in (i..=m - n).step_by(n) {
                if j - i >= concat_len {
                    hash = s_sub_hash[j - concat_len];
                    if s_sub_hash_count[&hash] == *words_hash_count.get(&hash).unwrap_or(&0) {
                        distinct_count -= 1;
                    }
                    *s_sub_hash_count.get_mut(&hash).unwrap() -= 1;
                }
                hash = s_sub_hash[j];
                *s_sub_hash_count.entry(hash).or_insert(0) += 1;
                if s_sub_hash_count[&hash] == *words_hash_count.get(&hash).unwrap_or(&0) {
                    distinct_count += 1;
                }

                if distinct_count == words_hash_count.len() {
                    ret.push((j + n - concat_len) as i32);
                }
            }
        }

        ret
    }
}
```
