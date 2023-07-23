# 2068. Check Whether Two Strings are Almost Equivalent
Two strings `word1` and `word2` are considered **almost equivalent** if the differences between the frequencies of each letter from `'a'` to `'z'` between `word1` and `word2` is **at most** `3`.

Given two strings `word1` and `word2`, each of length `n`, return `true` *if* `word1` *and* `word2` *are **almost equivalent**, or* `false` *otherwise*.

The **frequency** of a letter `x` is the number of times it occurs in the string.

#### Example 1:
<pre>
<strong>Input:</strong> word1 = "aaaa", word2 = "bccb"
<strong>Output:</strong> false
<strong>Explanation:</strong> There are 4 'a's in "aaaa" but 0 'a's in "bccb".
The difference is 4, which is more than the allowed 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> word1 = "abcdeef", word2 = "abaaacc"
<strong>Output:</strong> true
<strong>Explanation:</strong> The differences between the frequencies of each letter in word1 and word2 are at most 3:
- 'a' appears 1 time in word1 and 4 times in word2. The difference is 3.
- 'b' appears 1 time in word1 and 1 time in word2. The difference is 0.
- 'c' appears 1 time in word1 and 2 times in word2. The difference is 1.
- 'd' appears 1 time in word1 and 0 times in word2. The difference is 1.
- 'e' appears 2 times in word1 and 0 times in word2. The difference is 2.
- 'f' appears 1 time in word1 and 0 times in word2. The difference is 1.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> word1 = "cccddabba", word2 = "babababab"
<strong>Output:</strong> true
<strong>Explanation:</strong> The differences between the frequencies of each letter in word1 and word2 are at most 3:
- 'a' appears 2 times in word1 and 4 times in word2. The difference is 2.
- 'b' appears 2 times in word1 and 5 times in word2. The difference is 3.
- 'c' appears 3 times in word1 and 0 times in word2. The difference is 3.
- 'd' appears 2 times in word1 and 0 times in word2. The difference is 2.
</pre>

#### Constraints:
* `n == word1.length == word2.length`
* `1 <= n <= 100`
* `word1` and `word2` consist only of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let mut count = [0_i32; 26];

        for i in 0..word1.len() {
            count[(word1[i] - b'a') as usize] += 1;
            count[(word2[i] - b'a') as usize] -= 1;
        }

        count.iter().all(|&x| x.abs() <= 3)
    }
}
```
