# 1974. Minimum Time to Type Word Using Special Typewriter
There is a special typewriter with lowercase English letters `'a'` to `'z'` arranged in a **circle** with a **pointer**. A character can **only** be typed if the pointer is pointing to that character. The pointer is **initially** pointing to the character `'a'`.

![](https://assets.leetcode.com/uploads/2021/07/31/chart.jpg)

Each second, you may perform one of the following operations:
* Move the pointer one character **counterclockwise** or **clockwise**.
* Type the character the pointer is **currently** on.

Given a string `word`, return the **minimum** number of seconds to type out the characters in `word`.

#### Example 1:
<pre>
<strong>Input:</strong> word = "abc"
<strong>Output:</strong> 5
<strong>Explanation:</strong>
The characters are printed as follows:
- Type the character 'a' in 1 second since the pointer is initially on 'a'.
- Move the pointer clockwise to 'b' in 1 second.
- Type the character 'b' in 1 second.
- Move the pointer clockwise to 'c' in 1 second.
- Type the character 'c' in 1 second.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> word = "bza"
<strong>Output:</strong> 7
<strong>Explanation:</strong>
The characters are printed as follows:
- Move the pointer clockwise to 'b' in 1 second.
- Type the character 'b' in 1 second.
- Move the pointer counterclockwise to 'z' in 2 seconds.
- Type the character 'z' in 1 second.
- Move the pointer clockwise to 'a' in 1 second.
- Type the character 'a' in 1 second.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> word = "zjpc"
<strong>Output:</strong> 34
<strong>Explanation:</strong>
The characters are printed as follows:
- Move the pointer counterclockwise to 'z' in 1 second.
- Type the character 'z' in 1 second.
- Move the pointer clockwise to 'j' in 10 seconds.
- Type the character 'j' in 1 second.
- Move the pointer clockwise to 'p' in 6 seconds.
- Type the character 'p' in 1 second.
- Move the pointer counterclockwise to 'c' in 13 seconds.
- Type the character 'c' in 1 second.
</pre>

#### Constraints:
* `1 <= word.length <= 100`
* `word` consists of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        let mut curr = b'a' as i32;
        let mut ret = 0;

        for c in word.bytes() {
            let (s, l) = (curr.min(c as i32), curr.max(c as i32));
            ret += 1 + (l - s).min(s + 26 - l);
            curr = c as i32;
        }

        ret
    }
}
```
