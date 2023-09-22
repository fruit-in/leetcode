# 2182. 构造限制重复的字符串
给你一个字符串 `s` 和一个整数 `repeatLimit` ，用 `s` 中的字符构造一个新字符串 `repeatLimitedString` ，使任何字母 **连续** 出现的次数都不超过 `repeatLimit` 次。你不必使用 `s` 中的全部字符。

返回 **字典序最大的** `repeatLimitedString` 。

如果在字符串 `a` 和 `b` 不同的第一个位置，字符串 `a` 中的字母在字母表中出现时间比字符串 `b` 对应的字母晚，则认为字符串 `a` 比字符串 `b` **字典序更大** 。如果字符串中前 `min(a.length, b.length)` 个字符都相同，那么较长的字符串字典序更大。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "cczazcc", repeatLimit = 3
<strong>输出:</strong> "zzcccac"
<strong>解释:</strong> 使用 s 中的所有字符来构造 repeatLimitedString "zzcccac"。
字母 'a' 连续出现至多 1 次。
字母 'c' 连续出现至多 3 次。
字母 'z' 连续出现至多 2 次。
因此，没有字母连续出现超过 repeatLimit 次，字符串是一个有效的 repeatLimitedString 。
该字符串是字典序最大的 repeatLimitedString ，所以返回 "zzcccac" 。
注意，尽管 "zzcccca" 字典序更大，但字母 'c' 连续出现超过 3 次，所以它不是一个有效的 repeatLimitedString 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "aababab", repeatLimit = 2
<strong>输出:</strong> "bbabaa"
<strong>解释:</strong>
使用 s 中的一些字符来构造 repeatLimitedString "bbabaa"。
字母 'a' 连续出现至多 2 次。
字母 'b' 连续出现至多 2 次。
因此，没有字母连续出现超过 repeatLimit 次，字符串是一个有效的 repeatLimitedString 。
该字符串是字典序最大的 repeatLimitedString ，所以返回 "bbabaa" 。
注意，尽管 "bbabaaa" 字典序更大，但字母 'a' 连续出现超过 2 次，所以它不是一个有效的 repeatLimitedString 。
</pre>

#### 提示:
* <code>1 <= repeatLimit <= s.length <= 10<sup>5</sup></code>
* `s` 由小写英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut count = vec![0; 26];
        let mut repeat = 0;
        let mut flag = true;
        let mut ret = vec![];

        for c in s.bytes() {
            count[(c - b'a') as usize] += 1;
        }

        while flag {
            flag = false;

            for c in (b'a'..=b'z').rev() {
                if count[(c - b'a') as usize] > 0 {
                    if *ret.last().unwrap_or(&b' ') != c || repeat < repeat_limit {
                        if *ret.last().unwrap_or(&b' ') != c {
                            repeat = 0;
                        }
                        count[(c - b'a') as usize] -= 1;
                        repeat += 1;
                        flag = true;
                        ret.push(c);
                        break;
                    }
                }
            }
        }

        String::from_utf8(ret).unwrap()
    }
}
```
