# 1209. Remove All Adjacent Duplicates in String II
Given a string `s`, a *k* *duplicate removal* consists of choosing `k` adjacent and equal letters from `s` and removing them causing the left and the right side of the deleted substring to concatenate together.

We repeatedly make `k` duplicate removals on `s` until we no longer can.

Return the final string after all such duplicate removals have been made.

It is guaranteed that the answer is unique.

#### Example 1:
<pre>
<strong>Input:</strong> s = "abcd", k = 2
<strong>Output:</strong> "abcd"
<strong>Explanation:</strong> There's nothing to delete.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "deeedbbcccbdaa", k = 3
<strong>Output:</strong> "aa"
<strong>Explanation:</strong>
First delete "eee" and "ccc", get "ddbbbdaa"
Then delete "bbb", get "dddaa"
Finally delete "ddd", get "aa"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "pbbcggttciiippooaais", k = 2
<strong>Output:</strong> "ps"
</pre>

#### Constraints:
* `1 <= s.length <= 10^5`
* `2 <= k <= 10^4`
* `s` only contains lower case English letters.

## Solutions (Rust)

### 1. Stack
```Rust
impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack: Vec<(u8, i32)> = vec![];

        for ch in s.bytes() {
            match stack.last_mut() {
                Some(last) if ch == last.0 => {
                    last.1 += 1;
                    if last.1 == k {
                        stack.pop();
                    }
                }
                _ => stack.push((ch, 1)),
            }
        }

        String::from_utf8(
            stack
                .into_iter()
                .map(|(ch, cnt)| vec![ch; cnt as usize])
                .flatten()
                .collect(),
        )
        .unwrap()
    }
}
```
