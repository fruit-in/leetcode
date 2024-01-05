# 767. 重构字符串
给定一个字符串 `s` ，检查是否能重新排布其中的字母，使得两相邻的字符不同。

返回 *`s` 的任意可能的重新排列。若不可行，返回空字符串 `""`* 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "aab"
<strong>输出:</strong> "aba"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "aaab"
<strong>输出:</strong> ""
</pre>

#### 提示:
* `1 <= s.length <= 500`
* `s` 只包含小写字母

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut count = [0; 26];
        let mut heap = BinaryHeap::new();
        let mut ret = vec![];

        for ch in s.bytes() {
            count[(ch - b'a') as usize] += 1;
        }
        for ch in b'a'..=b'z' {
            heap.push((count[(ch - b'a') as usize], ch));
        }

        for _ in 0..s.len() {
            let (count0, ch0) = heap.pop().unwrap();

            if *ret.last().unwrap_or(&0) != ch0 {
                ret.push(ch0);
                heap.push((count0 - 1, ch0));
            } else if heap.peek().unwrap().0 == 0 {
                return String::new();
            } else {
                let (count1, ch1) = heap.pop().unwrap();
                ret.push(ch1);
                heap.push((count1 - 1, ch1));
                heap.push((count0, ch0));
            }
        }

        String::from_utf8(ret).unwrap()
    }
}
```
