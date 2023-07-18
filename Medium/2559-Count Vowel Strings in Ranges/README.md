# 2559. Count Vowel Strings in Ranges
You are given a **0-indexed** array of strings `words` and a 2D array of integers `queries`.

Each query <code>queries[i] = [l<sub>i</sub>, r<sub>i</sub>]</code> asks us to find the number of strings present in the range <code>l<sub>i</sub></code> to <code>r<sub>i</sub></code> (both **inclusive**) of `words` that start and end with a vowel.

Return *an array* `ans` *of size* `queries.length`, *where* `ans[i]` *is the answer to the* <code>i<sup>th</sup></code> *query*.

**Note** that the vowel letters are `'a'`, `'e'`, `'i'`, `'o'`, and `'u'`.

#### Example 1:
<pre>
<strong>Input:</strong> words = ["aba","bcb","ece","aa","e"], queries = [[0,2],[1,4],[1,1]]
<strong>Output:</strong> [2,3,0]
<strong>Explanation:</strong> The strings starting and ending with a vowel are "aba", "ece", "aa" and "e".
The answer to the query [0,2] is 2 (strings "aba" and "ece").
to query [1,4] is 3 (strings "ece", "aa", "e").
to query [1,1] is 0.
We return [2,3,0].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> words = ["a","e","i"], queries = [[0,2],[0,1],[2,2]]
<strong>Output:</strong> [3,2,1]
<strong>Explanation:</strong> Every string satisfies the conditions, so we return [3,2,1].
</pre>

#### Constraints:
* <code>1 <= words.length <= 10<sup>5</sup></code>
* `1 <= words[i].length <= 40`
* `words[i]` consists only of lowercase English letters.
* <code>sum(words[i].length) <= 3 * 10<sup>5</sup></code>
* <code>1 <= queries.length <= 10<sup>5</sup></code>
* <code>0 <= l<sub>i</sub> <= r<sub>i</sub> < words.length</code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prefix_sum = vec![0; words.len() + 1];
        let mut ret = vec![0; queries.len()];

        for i in 0..words.len() {
            prefix_sum[i + 1] = prefix_sum[i];

            if words[i].starts_with(|c| "aeiou".contains(c))
                && words[i].ends_with(|c| "aeiou".contains(c))
            {
                prefix_sum[i + 1] += 1;
            }
        }

        for i in 0..queries.len() {
            ret[i] = prefix_sum[queries[i][1] as usize + 1] - prefix_sum[queries[i][0] as usize];
        }

        ret
    }
}
```
