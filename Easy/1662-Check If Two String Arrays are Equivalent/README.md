# 1662. Check If Two String Arrays are Equivalent
Given two string arrays `word1` and `word2`, return `true` *if the two arrays **represent** the same string, and* `false` *otherwise*.

A string is **represented** by an array if the array elements concatenated **in order** forms the string.

#### Example 1:
<pre>
<strong>Input:</strong> word1 = ["ab", "c"], word2 = ["a", "bc"]
<strong>Output:</strong> true
<strong>Explanation:</strong>
word1 represents string "ab" + "c" -> "abc"
word2 represents string "a" + "bc" -> "abc"
The strings are the same, so return true.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> word1 = ["a", "cb"], word2 = ["ab", "c"]
<strong>Output:</strong> false
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> word1  = ["abc", "d", "defg"], word2 = ["abcddefg"]
<strong>Output:</strong> true
</pre>

#### Constraints:
* <code>1 <= word1.length, word2.length <= 10<sup>3</sup></code>
* <code>1 <= word1[i].length, word2[i].length <= 10<sup>3</sup></code>
* <code>1 <= sum(word1[i].length), sum(word2[i].length) <= 10<sup>3</sup></code>
* `word1[i]` and `word2[i]` consist of lowercase letters.

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {String[]} word1
# @param {String[]} word2
# @return {Boolean}
def array_strings_are_equal(word1, word2)
  word1.join == word2.join
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.concat() == word2.concat()
    }
}
```
