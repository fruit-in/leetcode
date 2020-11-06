# 791. Custom Sort String
`S` and `T` are strings composed of lowercase letters. In `S`, no letter occurs more than once.

`S` was sorted in some custom order previously. We want to permute the characters of `T` so that they match the order that `S` was sorted. More specifically, if `x` occurs before `y` in `S`, then `x` should occur before `y` in the returned string.

Return any permutation of `T` (as a string) that satisfies this property.

#### Example:
<pre>
<b>Input:</b>
S = "cba"
T = "abcd"
<b>Output:</b> "cbad"
<b>Explanation:</b>
"a", "b", "c" appear in S, so the order of "a", "b", "c" should be "c", "b", and "a".
Since "d" does not appear in S, it can be at any position in T. "dcba", "cdba", "cbda" are also valid outputs.
</pre>

#### Note:
* `S` has length at most `26`, and no character is repeated in `S`.
* `T` has length at most `200`.
* `S` and `T` consist of lowercase letters only.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn custom_sort_string(s: String, t: String) -> String {
        let mut indices = [26; 26];
        let mut t = t.into_bytes();

        for (i, ch) in s.bytes().enumerate() {
            indices[(ch - b'a') as usize] = i;
        }
        t.sort_unstable_by_key(|k| indices[(k - b'a') as usize]);

        String::from_utf8(t).unwrap()
    }
}
```
