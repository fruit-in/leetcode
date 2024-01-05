# 767. Reorganize String
Given a string `s`, rearrange the characters of `s` so that any two adjacent characters are not the same.

Return *any possible rearrangement of* `s` *or return* `""` *if not possible*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "aab"
<strong>Output:</strong> "aba"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "aaab"
<strong>Output:</strong> ""
</pre>

#### Constraints:
* `1 <= s.length <= 500`
* `s` consists of lowercase English letters.

## Solutions (Rust)

### 1. Solution
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
