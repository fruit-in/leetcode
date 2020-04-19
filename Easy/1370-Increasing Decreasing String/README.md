# 1370. Increasing Decreasing String
Given a string ```s```. You should re-order the string using the following algorithm:
1. Pick the **smallest** character from ```s``` and **append** it to the result.
2. Pick the **smallest** character from ```s``` which is greater than the last appended character to the result and **append** it.
3. Repeat step 2 until you cannot pick more characters.
4. Pick the **largest** character from ```s``` and **append** it to the result.
5. Pick the **largest** character from ```s``` which is smaller than the last appended character to the result and **append** it.
6. Repeat step 5 until you cannot pick more characters.
7. Repeat the steps from 1 to 6 until you pick all characters from ```s```.

In each step, If the smallest or the largest character appears more than once you can choose any occurrence and append it to the result.

Return *the result string* after sorting ```s``` with this algorithm.

#### Example 1:
<pre>
<strong>Input:</strong> s = "aaaabbbbcccc"
<strong>Output:</strong> "abccbaabccba"
<strong>Explanation:</strong> After steps 1, 2 and 3 of the first iteration, result = "abc"
After steps 4, 5 and 6 of the first iteration, result = "abccba"
First iteration is done. Now s = "aabbcc" and we go back to step 1
After steps 1, 2 and 3 of the second iteration, result = "abccbaabc"
After steps 4, 5 and 6 of the second iteration, result = "abccbaabccba"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "rat"
<strong>Output:</strong> "art"
<strong>Explanation:</strong> The word "rat" becomes "art" after re-ordering it with the mentioned algorithm.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "leetcode"
<strong>Output:</strong> "cdelotee"
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> s = "ggggggg"
<strong>Output:</strong> "ggggggg"
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> s = "spo"
<strong>Output:</strong> "ops"
</pre>

#### Constraints:
* ```1 <= s.length <= 500```
* ```s``` contains only lower-case English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut cnt = [0; 26];
        let mut ret = Vec::new();

        for ch in s.bytes() {
            cnt[(ch - b'a') as usize] += 1;
        }

        while ret.len() < s.len() {
            for i in 0..26 {
                if cnt[i] > 0 {
                    ret.push(i as u8 + b'a');
                    cnt[i] -= 1;
                }
            }
            for i in (0..26).rev() {
                if cnt[i] > 0 {
                    ret.push(i as u8 + b'a');
                    cnt[i] -= 1;
                }
            }
        }

        String::from_utf8(ret).unwrap()
    }
}
```
