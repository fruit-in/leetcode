# 1405. Longest Happy String
A string is called *happy* if it does not have any of the strings `'aaa'`, `'bbb'` or `'ccc'` as a substring.

Given three integers `a`, `b` and `c`, return **any** string `s`, which satisfies following conditions:
* `s` is *happy* and longest possible.
* `s` contains **at most** `a` occurrences of the letter `'a'`, **at most** `b` occurrences of the letter `'b'` and **at most** `c` occurrences of the letter `'c'`.
* `s` will only contain `'a'`, `'b'` and `'c'` letters.

If there is no such string `s` return the empty string `""`.

#### Example 1:
<pre>
<strong>Input:</strong> a = 1, b = 1, c = 7
<strong>Output:</strong> "ccaccbcc"
<strong>Explanation:</strong> "ccbccacc" would also be a correct answer.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> a = 2, b = 2, c = 1
<strong>Output:</strong> "aabbc"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> a = 7, b = 1, c = 0
<strong>Output:</strong> "aabaa"
<strong>Explanation:</strong> It's the only correct answer in this case.
</pre>

#### Constraints:
* `0 <= a, b, c <= 100`
* `a + b + c > 0`

## Solutions (Rust)

### 1. Greedy
```Rust
impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut abc = vec![a, b, c];
        let mut prev = 0;
        let mut v = vec![(a, 'a'), (b, 'b'), (c, 'c')];
        let mut ret = vec![];

        while let Some((mut x, ch)) = v.into_iter().filter(|t| t.0 > 0).max_by_key(|t| t.0) {
            ret.push(ch);
            x -= 1;
            if x >= prev && x > 0 {
                ret.push(ch);
                x -= 1;
            }

            abc[ch as usize - 97] = x;
            prev = x;
            v = match ch {
                'a' => vec![(abc[1], 'b'), (abc[2], 'c')],
                'b' => vec![(abc[0], 'a'), (abc[2], 'c')],
                _ => vec![(abc[0], 'a'), (abc[1], 'b')],
            };
        }

        ret.iter().collect()
    }
}
```
