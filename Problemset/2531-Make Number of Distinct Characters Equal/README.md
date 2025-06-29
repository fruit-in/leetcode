# 2531. Make Number of Distinct Characters Equal
You are given two **0-indexed** strings `word1` and `word2`.

A **move** consists of choosing two indices `i` and `j` such that `0 <= i < word1.length` and `0 <= j < word2.length` and swapping `word1[i]` with `word2[j]`.

Return `true` *if it is possible to get the number of distinct characters in* `word1` *and* `word2` *to be equal with **exactly one** move*. Return `false` *otherwise*.

#### Example 1:
<pre>
<strong>Input:</strong> word1 = "ac", word2 = "b"
<strong>Output:</strong> false
<strong>Explanation:</strong> Any pair of swaps would yield two distinct characters in the first string, and one in the second string.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> word1 = "abcc", word2 = "aab"
<strong>Output:</strong> true
<strong>Explanation:</strong> We swap index 2 of the first string with index 0 of the second string. The resulting strings are word1 = "abac" and word2 = "cab", which both have 3 distinct characters.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> word1 = "abcde", word2 = "fghij"
<strong>Output:</strong> true
<strong>Explanation:</strong> Both resulting strings will have 5 distinct characters, regardless of which indices we swap.
</pre>

#### Constraints:
* <code>1 <= word1.length, word2.length <= 10<sup>5</sup></code>
* `word1` and `word2` consist of only lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn is_it_possible(word1: String, word2: String) -> bool {
        let mut count1 = [0; 26];
        let mut count2 = [0; 26];
        let mut distinct1 = 0;
        let mut distinct2 = 0;

        for c in word1.bytes() {
            count1[(c - b'a') as usize] += 1;
            if count1[(c - b'a') as usize] == 1 {
                distinct1 += 1;
            }
        }
        for c in word2.bytes() {
            count2[(c - b'a') as usize] += 1;
            if count2[(c - b'a') as usize] == 1 {
                distinct2 += 1;
            }
        }

        for i in 0..26 {
            if count1[i] == 0 {
                continue;
            }

            for j in 0..26 {
                if count2[j] == 0 {
                    continue;
                }
                if i == j {
                    if distinct1 == distinct2 {
                        return true;
                    } else {
                        continue;
                    }
                }

                let mut tmp1 = distinct1;
                let mut tmp2 = distinct2;

                if count1[i] == 1 {
                    tmp1 -= 1;
                }
                if count2[i] == 0 {
                    tmp2 += 1;
                }
                if count1[j] == 0 {
                    tmp1 += 1;
                }
                if count2[j] == 1 {
                    tmp2 -= 1;
                }

                if tmp1 == tmp2 {
                    return true;
                }
            }
        }

        false
    }
}
```
