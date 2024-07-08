# 1781. 所有子字符串美丽值之和
一个字符串的 **美丽值** 定义为：出现频率最高字符与出现频率最低字符的出现次数之差。

* 比方说，`"abaacc"` 的美丽值为 `3 - 1 = 2` 。

给你一个字符串 `s` ，请你返回它所有子字符串的 **美丽值** 之和。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "aabcb"
<strong>输出:</strong> 5
<strong>解释:</strong> 美丽值不为零的字符串包括 ["aab","aabc","aabcb","abcb","bcb"] ，每一个字符串的美丽值都为 1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "aabcbaa"
<strong>输出:</strong> 17
</pre>

#### 提示:
* `1 <= s.length <= 500`
* `s` 只包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn beauty_sum(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ret = 0;

        for i in 0..s.len() {
            let mut count = [0; 26];

            for j in i..s.len() {
                count[(s[j] - b'a') as usize] += 1;

                let max_freq = count.iter().max().unwrap();
                let min_freq = count.iter().filter(|&&x| x > 0).min().unwrap();

                ret += max_freq - min_freq;
            }
        }

        ret
    }
}
```
