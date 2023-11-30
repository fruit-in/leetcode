# 1156. Swap For Longest Repeated Character Substring
You are given a string `text`. You can swap two of the characters in the `text`.

Return *the length of the longest substring with repeated characters*.

#### Example 1:
<pre>
<strong>Input:</strong> text = "ababa"
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can swap the first 'b' with the last 'a', or the last 'b' with the first 'a'. Then, the longest repeated character substring is "aaa" with length 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> text = "aaabaaa"
<strong>Output:</strong> 6
<strong>Explanation:</strong> Swap 'b' with the last 'a' (or the first 'a'), and we get longest repeated character substring "aaaaaa" with length 6.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> text = "aaaaa"
<strong>Output:</strong> 5
<strong>Explanation:</strong> No need to swap, longest repeated character substring is "aaaaa" with length is 5.
</pre>

#### Constraints:
* <code>1 <= text.length <= 2 * 10<sup>4</sup></code>
* `text` consist of lowercase English characters only.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_rep_opt1(text: String) -> i32 {
        let text = text.as_bytes();
        let mut ranges = vec![vec![]; 26];
        let mut ret = 1;

        for i in 0..text.len() {
            if i == 0 || text[i] != text[i - 1] {
                ranges[(text[i] - b'a') as usize].push((i, i));
            } else {
                ranges[(text[i] - b'a') as usize].last_mut().unwrap().1 = i;
            }
        }

        for i in 0..ranges.len() {
            for j in 0..ranges[i].len() {
                ret = ret.max(ranges[i][j].1 - ranges[i][j].0 + 1 + (ranges[i].len() > 1) as usize);

                if j > 0 && ranges[i][j].0 == ranges[i][j - 1].1 + 2 {
                    ret = ret
                        .max(ranges[i][j].1 - ranges[i][j - 1].0 + (ranges[i].len() > 2) as usize);
                }
            }
        }

        ret as i32
    }
}
```
