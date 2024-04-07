# 899. 有序队列
给定一个字符串 `s` 和一个整数 `k` 。你可以从 `s` 的前 `k` 个字母中选择一个，并把它加到字符串的末尾。

返回 *在应用上述步骤的任意数量的移动后，字典序最小的字符串* 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "cba", k = 1
<strong>输出:</strong> "acb"
<strong>解释:</strong>
在第一步中，我们将第一个字符（“c”）移动到最后，获得字符串 “bac”。
在第二步中，我们将第一个字符（“b”）移动到最后，获得最终结果 “acb”。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "baaca", k = 3
<strong>输出:</strong> "aaabc"
<strong>解释:</strong>
在第一步中，我们将第一个字符（“b”）移动到最后，获得字符串 “aacab”。
在第二步中，我们将第三个字符（“c”）移动到最后，获得最终结果 “aaabc”。
</pre>

#### 提示:
* `1 <= k <= s.length <= 1000`
* `s` 只由小写字母组成。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        let mut s = s.into_bytes();
        let n = s.len();
        let mut i = 0;

        if k > 1 {
            s.sort_unstable();
        } else {
            for j in 1..n {
                for k in 0..n {
                    if s[(i + k) % n] < s[(j + k) % n] {
                        break;
                    } else if s[(i + k) % n] > s[(j + k) % n] {
                        i = j;
                        break;
                    }
                }
            }

            s.rotate_left(i);
        }

        String::from_utf8(s).unwrap()
    }
}
```
