# 2573. 找出对应 LCP 矩阵的字符串
对任一由 `n` 个小写英文字母组成的字符串 `word` ，我们可以定义一个 `n x n` 的矩阵，并满足：
* `lcp[i][j]` 等于子字符串 `word[i,...,n-1]` 和 `word[j,...,n-1]` 之间的最长公共前缀的长度。

给你一个 `n x n` 的矩阵 `lcp` 。返回与 `lcp` 对应的、按字典序最小的字符串 `word` 。如果不存在这样的字符串，则返回空字符串。

对于长度相同的两个字符串 `a` 和 `b` ，如果在 `a` 和 `b` 不同的第一个位置，字符串 `a` 的字母在字母表中出现的顺序先于 `b` 中的对应字母，则认为字符串 `a` 按字典序比字符串 `b` 小。例如，`"aabd"` 在字典上小于 `"aaca"` ，因为二者不同的第一位置是第三个字母，而 `'b'` 先于 `'c'` 出现。

#### 示例 1:
<pre>
<strong>输入:</strong> lcp = [[4,0,2,0],[0,3,0,1],[2,0,2,0],[0,1,0,1]]
<strong>输出:</strong> "abab"
<strong>解释:</strong> lcp 对应由两个交替字母组成的任意 4 字母字符串，字典序最小的是 "abab" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> lcp = [[4,3,2,1],[3,3,2,1],[2,2,2,1],[1,1,1,1]]
<strong>输出:</strong> "aaaa"
<strong>解释:</strong> lcp 对应只有一个不同字母的任意 4 字母字符串，字典序最小的是 "aaaa" 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> lcp = [[4,3,2,1],[3,3,2,1],[2,2,2,1],[1,1,1,3]]
<strong>输出:</strong> ""
<strong>解释:</strong> lcp[3][3] 无法等于 3 ，因为 word[3,...,3] 仅由单个字母组成；因此，不存在答案。
</pre>

#### 提示:
* `1 <= n == lcp.length == lcp[i].length <= 1000`
* `0 <= lcp[i][j] <= n`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        let n = lcp.len();
        let mut masks = vec![0_i32; n];
        let mut word = vec![b'a'; n];

        for i in 0..n {
            if lcp[i][i] as usize != n - i || masks[i].trailing_ones() > 25 {
                return "".to_string();
            }

            word[i] += masks[i].trailing_ones() as u8;

            for j in i + 1..n {
                if lcp[i][j] > 0 && i + 1 < n && j + 1 < n && lcp[i + 1][j + 1] != lcp[i][j] - 1 {
                    return "".to_string();
                } else if lcp[i][j] != lcp[j][i] {
                    return "".to_string();
                } else if lcp[i][j] as usize > n - i.max(j) {
                    return "".to_string();
                }

                masks[j] |= match lcp[i][j] {
                    0 => 1 << (word[i] - b'a'),
                    _ => i32::MAX ^ (1 << (word[i] - b'a')),
                };
            }
        }

        String::from_utf8(word).unwrap()
    }
}
```
