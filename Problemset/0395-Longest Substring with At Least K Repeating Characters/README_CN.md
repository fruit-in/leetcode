# 395. 至少有 K 个重复字符的最长子串
给你一个字符串 `s` 和一个整数 `k` ，请你找出 `s` 中的最长子串， 要求该子串中的每一字符出现次数都不少于 `k` 。返回这一子串的长度。

如果不存在这样的子字符串，则返回 0。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "aaabb", k = 3
<strong>输出:</strong> 3
<strong>解释:</strong> 最长子串为 "aaa" ，其中 'a' 重复了 3 次。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "ababbc", k = 2
<strong>输出:</strong> 5
<strong>解释:</strong> 最长子串为 "ababb" ，其中 'a' 重复了 2 次， 'b' 重复了 3 次。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>4</sup></code>
* `s` 仅由小写英文字母组成
* <code>1 <= k <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        Self::dfs(s.as_bytes(), 0, s.len() - 1, k as usize)
    }

    fn dfs(s: &[u8], l: usize, r: usize, k: usize) -> i32 {
        if l > r || r - l + 1 < k {
            return 0;
        }

        let mut indices = vec![vec![]; 26];
        let mut ret = (r - l + 1) as i32;

        for i in l..=r {
            indices[(s[i] - b'a') as usize].push(i);
        }

        for i in 0..26 {
            if !indices[i].is_empty() && indices[i].len() < k {
                ret = 0;
                ret = ret.max(Self::dfs(s, l, indices[i][0].saturating_sub(1), k));
                for j in 1..indices[i].len() {
                    ret = ret.max(Self::dfs(s, indices[i][j - 1] + 1, indices[i][j] - 1, k));
                }
                ret = ret.max(Self::dfs(s, *indices[i].last().unwrap() + 1, r, k));
                break;
            }
        }

        ret
    }
}
```
