# 1750. Minimum Length of String After Deleting Similar Ends
Given a string `s` consisting only of characters `'a'`, `'b'`, and `'c'`. You are asked to apply the following algorithm on the string any number of times:
1. Pick a **non-empty** prefix from the string `s` where all the characters in the prefix are equal.
2. Pick a **non-empty** suffix from the string `s` where all the characters in this suffix are equal.
3. The prefix and the suffix should not intersect at any index.
4. The characters from the prefix and suffix must be the same.
5. Delete both the prefix and the suffix.

Return *the **minimum length** of* `s` *after performing the above operation any number of times (possibly zero times)*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "ca"
<strong>Output:</strong> 2
<strong>Explanation:</strong> You can't remove any characters, so the string stays as is.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "cabaabac"
<strong>Output:</strong> 0
<strong>Explanation:</strong> An optimal sequence of operations is:
- Take prefix = "c" and suffix = "c" and remove them, s = "abaaba".
- Take prefix = "a" and suffix = "a" and remove them, s = "baab".
- Take prefix = "b" and suffix = "b" and remove them, s = "aa".
- Take prefix = "a" and suffix = "a" and remove them, s = "".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "aabccabba"
<strong>Output:</strong> 3
<strong>Explanation:</strong> An optimal sequence of operations is:
- Take prefix = "aa" and suffix = "a" and remove them, s = "bccabb".
- Take prefix = "b" and suffix = "bb" and remove them, s = "cca".
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` only consists of characters `'a'`, `'b'`, and `'c'`.

## Solutions (Ruby)

### 1. Two Pointers
```Ruby
# @param {String} s
# @return {Integer}
def minimum_length(s)
  i = 0
  j = s.size - 1

  while i < j && s[i] == s[j]
    c = s[i]
    i += 1 while i < j && s[i] == c
    return 0 if i == j
    j -= 1 while i < j && s[j] == c
  end

  j - i + 1
end
```

## Solutions (Rust)

### 1. Two Pointers
```Rust
impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let mut i = 0;
        let mut j = s.len() - 1;

        while i < j && s[i] == s[j] {
            let c = s[i];
            while i < j && s[i] == c {
                i += 1;
            }
            if i == j {
                return 0;
            }
            while i < j && s[j] == c {
                j -= 1;
            }
        }

        (j - i + 1) as i32
    }
}
```
