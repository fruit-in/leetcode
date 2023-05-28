# 1177. Can Make Palindrome from Substring
You are given a string `s` and array `queries` where <code>queries[i] = [left<sub>i</sub>, right<sub>i</sub>, k<sub>i</sub>]</code>. We may rearrange the substring <code>s[left<sub>i</sub>...right<sub>i</sub>]</code> for each query and then choose up to <code>k<sub>i</sub></code> of them to replace with any lowercase English letter.

If the substring is possible to be a palindrome string after the operations above, the result of the query is `true`. Otherwise, the result is `false`.

Return a boolean array `answer` where `answer[i]` is the result of the <code>i<sup>th</sup></code> query `queries[i]`.

Note that each letter is counted individually for replacement, so if, for example <code>s[left<sub>i</sub>...right<sub>i</sub>] = "aaa"</code>, and <code>k<sub>i</sub> = 2</code>, we can only replace two of the letters. Also, note that no query modifies the initial string `s`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "abcda", queries = [[3,3,0],[1,2,0],[0,3,1],[0,3,2],[0,4,1]]
<strong>Output:</strong> [true,false,false,true,true]
<strong>Explanation:</strong>
queries[0]: substring = "d", is palidrome.
queries[1]: substring = "bc", is not palidrome.
queries[2]: substring = "abcd", is not palidrome after replacing only 1 character.
queries[3]: substring = "abcd", could be changed to "abba" which is palidrome. Also this can be changed to "baab" first rearrange it "bacd" then replace "cd" with "ab".
queries[4]: substring = "abcda", could be changed to "abcba" which is palidrome.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "lyb", queries = [[0,1,0],[2,2,1]]
<strong>Output:</strong> [false,true]
</pre>

#### Constraints:
* <code>1 <= s.length, queries.length <= 10<sup>5</sup></code>
* <code>0 <= left<sub>i</sub> <= right<sub>i</sub> < s.length</code>
* <code>0 <= k<sub>i</sub> <= s.length</code>
* `s` consists of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut prefix = vec![0_i32; s.len() + 1];
        let mut answer = vec![true; queries.len()];

        for (i, c) in s.bytes().enumerate() {
            prefix[i + 1] = prefix[i] ^ (1 << (c - b'a'));
        }

        for i in 0..queries.len() {
            let left = queries[i][0] as usize;
            let right = queries[i][1] as usize;
            let k = queries[i][2] as u32;

            answer[i] = (prefix[left] ^ prefix[right + 1]).count_ones() / 2 <= k;
        }

        answer
    }
}
```
