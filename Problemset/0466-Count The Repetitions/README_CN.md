# 466. 统计重复个数
定义 `str = [s, n]` 表示 `str` 由 `n` 个字符串 `s` 连接构成。

* 例如，`str == ["abc", 3] =="abcabcabc"` 。

如果可以从 `s2` 中删除某些字符使其变为 `s1`，则称字符串 `s1` 可以从字符串 `s2` 获得。

* 例如，根据定义，`s1 = "abc"` 可以从 `s2 = "abdbec"` 获得，仅需要删除加粗且用斜体标识的字符。

现在给你两个字符串 `s1` 和 `s2` 和两个整数 `n1` 和 `n2` 。由此构造得到两个字符串，其中 `str1 = [s1, n1]`、`str2 = [s2, n2]` 。

请你找出一个最大整数 `m` ，以满足 `str = [str2, m]` 可以从 `str1` 获得。

#### 示例 1:
<pre>
<strong>输入:</strong> s1 = "acb", n1 = 4, s2 = "ab", n2 = 2
<strong>输出:</strong> 2
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s1 = "acb", n1 = 1, s2 = "acb", n2 = 1
<strong>输出:</strong> 1
</pre>

#### 提示:
* `1 <= s1.length, s2.length <= 100`
* `s1` 和 `s2` 由小写英文字母组成
* <code>1 <= n1, n2 <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut i = 0;
        let mut memo = HashMap::new();
        let mut count2 = 0;

        for _ in 0..n1 {
            if let Some(&(count, j)) = memo.get(&i) {
                count2 += count;
                i = j;
                continue;
            }

            let init = i;
            let mut count = 0;

            for j in 0..s1.len() {
                if s1[j] == s2[i] {
                    i = (i + 1) % s2.len();
                    if i == 0 {
                        count += 1;
                    }
                }
            }

            memo.insert(init, (count, i));
            count2 += count;
        }

        count2 / n2
    }
}
```
