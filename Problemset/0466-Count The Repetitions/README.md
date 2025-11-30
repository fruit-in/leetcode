# 466. Count The Repetitions
We define `str = [s, n]` as the string `str` which consists of the string `s` concatenated `n` times.

* For example, `str == ["abc", 3] =="abcabcabc"`.

We define that string `s1` can be obtained from string `s2` if we can remove some characters from `s2` such that it becomes `s1`.

* For example, `s1 = "abc"` can be obtained from `s2 = "abdbec"` based on our definition by removing the bolded underlined characters.

You are given two strings `s1` and `s2` and two integers `n1` and `n2`. You have the two strings `str1 = [s1, n1]` and `str2 = [s2, n2]`.

Return *the maximum integer* `m` *such that* `str = [str2, m]` *can be obtained from* `str1`.

#### Example 1:
<pre>
<strong>Input:</strong> s1 = "acb", n1 = 4, s2 = "ab", n2 = 2
<strong>Output:</strong> 2
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s1 = "acb", n1 = 1, s2 = "acb", n2 = 1
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `1 <= s1.length, s2.length <= 100`
* `s1` and `s2` consist of lowercase English letters.
* <code>1 <= n1, n2 <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
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
