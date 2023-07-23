# 316. 去除重复字母
给你一个字符串 `s` ，请你去除字符串中重复的字母，使得每个字母只出现一次。需保证 **返回结果的字典序最小**（要求不能打乱其他字符的相对位置）。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "bcabc"
<strong>输出:</strong> "abc"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "cbacdcbc"
<strong>输出:</strong> "acdb"
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>4</sup></code>
* `s` 由小写英文字母组成

**注意：**该题与 1081 https://leetcode-cn.com/problems/smallest-subsequence-of-distinct-characters 相同

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let s = s.as_bytes();
        let mut mask = 0;
        let mut masks = vec![0; s.len()];
        let mut indices = vec![vec![]; 26];
        let mut i = 0;
        let mut ret = vec![];

        for j in (0..s.len()).rev() {
            masks[j] = mask;
            mask |= 1 << (s[j] - b'a');
            indices[(s[j] - b'a') as usize].push(j);
        }

        while mask != 0 {
            for j in (0..=26).filter(|x| mask & (1 << x) != 0) {
                let new_mask = mask ^ (1 << j);
                let k = match indices[j].binary_search_by(|x| i.cmp(x)) {
                    Ok(y) => indices[j][y],
                    Err(y) => indices[j][y - 1],
                };

                if masks[k] & new_mask == new_mask {
                    mask = new_mask;
                    i = k;
                    ret.push(b'a' + j as u8);
                    break;
                }
            }
        }

        String::from_utf8(ret).unwrap()
    }
}
```
