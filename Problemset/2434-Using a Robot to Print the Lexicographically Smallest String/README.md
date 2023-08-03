# 2434. Using a Robot to Print the Lexicographically Smallest String
You are given a string `s` and a robot that currently holds an empty string `t`. Apply one of the following operations until `s` and `t` **are both empty**:

* Remove the **first** character of a string `s` and give it to the robot. The robot will append this character to the string `t`.
* Remove the **last** character of a string `t` and give it to the robot. The robot will write this character on paper.

Return *the lexicographically smallest string that can be written on the paper*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "zza"
<strong>Output:</strong> "azz"
<strong>Explanation:</strong> Let p denote the written string.
Initially p="", s="zza", t="".
Perform first operation three times p="", s="", t="zza".
Perform second operation three times p="azz", s="", t="".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "bac"
<strong>Output:</strong> "abc"
<strong>Explanation:</strong> Let p denote the written string.
Perform first operation twice p="", s="c", t="ba".
Perform second operation twice p="ab", s="c", t="".
Perform first operation p="ab", s="", t="c".
Perform second operation p="abc", s="", t="".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "bdda"
<strong>Output:</strong> "addb"
<strong>Explanation:</strong> Let p denote the written string.
Initially p="", s="bdda", t="".
Perform first operation four times p="", s="", t="bdda".
Perform second operation four times p="addb", s="", t="".
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` consists of only English lowercase letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let mut s = s.bytes().rev().collect::<Vec<_>>();
        let mut t = vec![];
        let mut p = vec![];
        let mut count = [0; 26];

        for &ch in &s {
            count[(ch - b'a') as usize] += 1;
        }

        while let Some(ch0) = s.pop() {
            count[(ch0 - b'a') as usize] -= 1;
            t.push(ch0);

            while let Some(&ch1) = t.last() {
                if count.iter().take((ch1 - b'a') as usize).all(|&x| x == 0) {
                    p.push(t.pop().unwrap());
                } else {
                    break;
                }
            }
        }

        while let Some(ch) = t.pop() {
            p.push(ch);
        }

        String::from_utf8(p).unwrap()
    }
}
```
