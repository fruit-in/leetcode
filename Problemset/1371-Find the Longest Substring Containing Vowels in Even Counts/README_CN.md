# 1371. 每个元音包含偶数次的最长子字符串
给你一个字符串 `s` ，请你返回满足以下条件的最长子字符串的长度：每个元音字母，即 'a'，'e'，'i'，'o'，'u' ，在子字符串中都恰好出现了偶数次。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "eleetminicoworoep"
<strong>输出:</strong> 13
<strong>解释:</strong> 最长子字符串是 "leetminicowor" ，它包含 e，i，o 各 2 个，以及 0 个 a，u 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "leetcodeisgreat"
<strong>输出:</strong> 5
<strong>解释:</strong> 最长子字符串是 "leetc" ，其中包含 2 个 e 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "bcbcbc"
<strong>输出:</strong> 6
<strong>解释:</strong> 这个示例中，字符串 "bcbcbc" 本身就是最长的，因为所有的元音 a，e，i，o，u 都出现了 0 次。
</pre>

#### 提示:
* `1 <= s.length <= 5 x 10^5`
* `s` 只包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut even = [true; 5];
        let mut first_appear = vec![(even, -1)].into_iter().collect::<HashMap<_, _>>();
        let mut ret = 0;

        for i in 0..s.len() {
            match s[i] {
                b'a' => even[0] = !even[0],
                b'e' => even[1] = !even[1],
                b'i' => even[2] = !even[2],
                b'o' => even[3] = !even[3],
                b'u' => even[4] = !even[4],
                _ => (),
            }

            match first_appear.get(&even) {
                Some(j) => ret = ret.max(i as i32 - j),
                None => {
                    first_appear.insert(even, i as i32);
                }
            }
        }

        ret
    }
}
```
