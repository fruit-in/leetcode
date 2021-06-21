# 1371. Find the Longest Substring Containing Vowels in Even Counts
Given the string `s`, return the size of the longest substring containing each vowel an even number of times. That is, 'a', 'e', 'i', 'o', and 'u' must appear an even number of times.

#### Example 1:
<pre>
<strong>Input:</strong> s = "eleetminicoworoep"
<strong>Output:</strong> 13
<strong>Explanation:</strong> The longest substring is "leetminicowor" which contains two each of the vowels: e, i and o and zero of the vowels: a and u.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "leetcodeisgreat"
<strong>Output:</strong> 5
<strong>Explanation:</strong> The longest substring is "leetc" which contains two e's.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "bcbcbc"
<strong>Output:</strong> 6
<strong>Explanation:</strong> In this case, the given string "bcbcbc" is the longest because all vowels: a, e, i, o and u appear zero times.
</pre>

#### Constraints:
* `1 <= s.length <= 5 x 10^5`
* `s` contains only lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut even = [true; 5];
        let mut first_appear = vec![(even, -1)].into_iter().collect::<HashMap<_, _>>();
        let mut ret = 0;

        for i in 0..s.len() {
            match s[i] {
                b'a' => even[0] = !even[0],
                b'e' => even[1] = !even[1],
                b'i' => even[2] = !even[2],
                b'o' => even[3] = !even[3],
                b'u' => even[4] = !even[4],
                _ => (),
            }

            match first_appear.get(&even) {
                Some(j) => ret = ret.max(i as i32 - j),
                None => {
                    first_appear.insert(even, i as i32);
                }
            }
        }

        ret
    }
}
```
