# 2262. Total Appeal of A String
The **appeal** of a string is the number of **distinct** characters found in the string.

* For example, the appeal of `"abbca"` is `3` because it has `3` distinct characters: `'a'`, `'b'`, and `'c'`.

Given a string `s`, return *the **total appeal of all of its substrings***.

A **substring** is a contiguous sequence of characters within a string.

#### Example 1:
<pre>
<strong>Input:</strong> s = "abbca"
<strong>Output:</strong> 28
<strong>Explanation:</strong> The following are the substrings of "abbca":
- Substrings of length 1: "a", "b", "b", "c", "a" have an appeal of 1, 1, 1, 1, and 1 respectively. The sum is 5.
- Substrings of length 2: "ab", "bb", "bc", "ca" have an appeal of 2, 1, 2, and 2 respectively. The sum is 7.
- Substrings of length 3: "abb", "bbc", "bca" have an appeal of 2, 2, and 3 respectively. The sum is 7.
- Substrings of length 4: "abbc", "bbca" have an appeal of 3 and 3 respectively. The sum is 6.
- Substrings of length 5: "abbca" has an appeal of 3. The sum is 3.
The total sum is 5 + 7 + 7 + 6 + 3 = 28.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "code"
<strong>Output:</strong> 20
<strong>Explanation:</strong> The following are the substrings of "code":
- Substrings of length 1: "c", "o", "d", "e" have an appeal of 1, 1, 1, and 1 respectively. The sum is 4.
- Substrings of length 2: "co", "od", "de" have an appeal of 2, 2, and 2 respectively. The sum is 6.
- Substrings of length 3: "cod", "ode" have an appeal of 3 and 3 respectively. The sum is 6.
- Substrings of length 4: "code" has an appeal of 4. The sum is 4.
The total sum is 4 + 6 + 6 + 4 = 20.
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` consists of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn appeal_sum(s: String) -> i64 {
        let s = s.as_bytes();
        let mut last = vec![-1; 27];
        let mut mask = 0_i32;
        let mut ret = 0;

        for i in 0..s.len() {
            last[(s[i] - b'a') as usize] = i as i64;
            mask |= 1 << (s[i] - b'a');
            let mut tmp = last.clone();
            let mut appeal = mask.count_ones() as i64;
            tmp.sort_unstable();

            for j in 1..27 {
                if tmp[j] > -1 {
                    ret += (tmp[j] - tmp[j - 1]) * appeal;
                    appeal -= 1;
                }
            }
        }

        ret
    }
}
```
