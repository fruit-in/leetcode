# 521. Longest Uncommon Subsequence I 
Given a group of two strings, you need to find the longest uncommon subsequence of this group of two strings. The longest uncommon subsequence is defined as the longest subsequence of one of these strings and this subsequence should not be **any** subsequence of the other strings.

A **subsequence** is a sequence that can be derived from one sequence by deleting some characters without changing the order of the remaining elements. Trivially, any string is a subsequence of itself and an empty string is a subsequence of any string.

The input will be two strings, and the output needs to be the length of the longest uncommon subsequence. If the longest uncommon subsequence doesn't exist, return -1.

#### Example 1:
<pre>
<strong>Input:</strong> "aba", "cdc"
<strong>Output:</strong> 3
<strong>Explanation:</strong> The longest uncommon subsequence is "aba" (or "cdc"), 
because "aba" is a subsequence of "aba", 
but not a subsequence of any other strings in the group of two strings. 
</pre>

#### Note:
1. Both strings' lengths will not exceed 100.
2. Only letters from a ~ z will appear in input strings. 

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {String} a
# @param {String} b
# @return {Integer}
def find_lu_slength(a, b)
    if a == b
        return -1
    else
        return [a.length, b.length].max
    end
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a == b {
            -1
        } else {
            a.len().max(b.len()) as i32
        }
    }
}
```
