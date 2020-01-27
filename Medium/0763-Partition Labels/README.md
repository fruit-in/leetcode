# 763. Partition Labels
A string ```S``` of lowercase letters is given. We want to partition this string into as many parts as possible so that each letter appears in at most one part, and return a list of integers representing the size of these parts.

#### Example 1:
<pre>
<strong>Input:</strong> S = "ababcbacadefegdehijhklij"
<strong>Output:</strong> [9,7,8]
<strong>Explanation:</strong>
The partition is "ababcbaca", "defegde", "hijhklij".
This is a partition so that each letter appears in at most one part.
A partition like "ababcbacadefegde", "hijhklij" is incorrect, because it splits S into less parts.
</pre>

#### Note:
1. ```S``` will have length in range ```[1, 500]```.
2. ```S``` will consist of lowercase letters (```'a'``` to ```'z'```) only.

## Solutions (Rust)

### 1. Greedy
```Rust
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last = [0; 26];

        for (i, ch) in s.bytes().enumerate() {
            last[(ch - b'a') as usize] = i;
        }

        let mut l = 0;
        let mut r = 0;
        let mut ret = Vec::new();

        for (i, ch) in s.bytes().enumerate() {
            if i > r {
                ret.push((r - l) as i32 + 1);
                l = i;
                r = last[(ch - b'a') as usize];
            } else if last[(ch - b'a') as usize] > r {
                r = last[(ch - b'a') as usize];
            }
        }

        ret.push((r - l) as i32 + 1);

        ret
    }
}
```
