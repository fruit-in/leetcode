# 2193. 得到回文串的最少操作次数
给你一个只包含小写英文字母的字符串 `s` 。

每一次 **操作** ，你可以选择 `s` 中两个 **相邻** 的字符，并将它们交换。

请你返回将 `s` 变成回文串的 **最少操作次数** 。

**注意** ，输入数据会确保 `s` 一定能变成一个回文串。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "aabb"
<strong>输出:</strong> 2
<strong>解释:</strong>
我们可以将 s 变成 2 个回文串，"abba" 和 "baab" 。
- 我们可以通过 2 次操作得到 "abba" ："aabb" -> "abab" -> "abba" 。
- 我们可以通过 2 次操作得到 "baab" ："aabb" -> "abab" -> "baab" 。
因此，得到回文串的最少总操作次数为 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "letelt"
<strong>输出:</strong> 2
<strong>解释:</strong>
通过 2 次操作从 s 能得到回文串 "lettel" 。
其中一种方法是："letelt" -> "letetl" -> "lettel" 。
其他回文串比方说 "tleelt" 也可以通过 2 次操作得到。
可以证明少于 2 次操作，无法得到回文串。
</pre>

#### 提示:
* `1 <= s.length <= 2000`
* `s` 只包含小写英文字母。
* `s` 可以通过有限次操作得到一个回文串。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_moves_to_make_palindrome(s: String) -> i32 {
        let mut s = s.into_bytes();
        let n = s.len();
        let mut l = 0;
        let mut r = n - 1;
        let mut ret = 0;

        while l < r {
            for i in (l..=r).rev() {
                if i == l {
                    ret += (n / 2 - i) as i32;
                } else if s[i] == s[l] {
                    for j in i..r {
                        s.swap(j, j + 1);
                        ret += 1;
                    }

                    r -= 1;
                    break;
                }
            }

            l += 1;
        }

        ret
    }
}
```
