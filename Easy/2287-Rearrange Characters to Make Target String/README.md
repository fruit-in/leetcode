# 2287. Rearrange Characters to Make Target String
You are given two **0-indexed** strings `s` and `target`. You can take some letters from `s` and rearrange them to form new strings.

Return *the **maximum** number of copies of* `target` *that can be formed by taking letters from* `s` *and rearranging them*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "ilovecodingonleetcode", target = "code"
<strong>Output:</strong> 2
<strong>Explanation:</strong>
For the first copy of "code", take the letters at indices 4, 5, 6, and 7.
For the second copy of "code", take the letters at indices 17, 18, 19, and 20.
The strings that are formed are "ecod" and "code" which can both be rearranged into "code".
We can make at most two copies of "code", so we return 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "abcba", target = "abc"
<strong>Output:</strong> 1
<strong>Explanation:</strong>
We can make one copy of "abc" by taking the letters at indices 0, 1, and 2.
We can make at most one copy of "abc", so we return 1.
Note that while there is an extra 'a' and 'b' at indices 3 and 4, we cannot reuse the letter 'c' at index 2, so we cannot make a second copy of "abc".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "abbaccaddaeea", target = "aaaaa"
<strong>Output:</strong> 1
<strong>Explanation:</strong>
We can make one copy of "aaaaa" by taking the letters at indices 0, 3, 6, 9, and 12.
We can make at most one copy of "aaaaa", so we return 1.
</pre>

#### Constraints:
* `1 <= s.length <= 100`
* `1 <= target.length <= 10`
* `s` and `target` consist of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let mut count_s = [0; 26];
        let mut count_t = [0; 26];

        s.bytes().for_each(|c| count_s[(c - b'a') as usize] += 1);
        target
            .bytes()
            .for_each(|c| count_t[(c - b'a') as usize] += 1);

        (0..26)
            .filter(|&i| count_t[i] > 0)
            .map(|i| count_s[i] / count_t[i])
            .min()
            .unwrap()
    }
}
```
