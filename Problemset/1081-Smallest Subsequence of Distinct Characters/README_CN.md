# 1081. 不同字符的最小子序列
返回 `s` 字典序最小的子序列，该子序列包含 `s` 的所有不同字符，且只包含一次。

**注意：**该题与 316 https://leetcode.com/problems/remove-duplicate-letters/ 相同

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
* `1 <= s.length <= 1000`
* `s` 由小写英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
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
                    ret.push(b'a' + j);
                }
            }
        }

        String::from_utf8(ret).unwrap()
    }
}
```
