# 392. Is Subsequence
Given a string **s** and a string **t**, check if **s** is subsequence of **t**.

You may assume that there is only lower case English letters in both **s** and **t**. **t** is potentially a very long (length ~= 500,000) string, and **s** is a short string (<=100).

A subsequence of a string is a new string which is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (ie, ```"ace"``` is a subsequence of ```"abcde"``` while ```"aec"``` is not).

#### Example 1:
**s** = ```"abc"```, **t** = ```"ahbgdc"```

Return ```true```.

#### Example 2:
**s** = ```"axc"```, **t** = ```"ahbgdc"```

Return ```false```.

#### Follow up:
If there are lots of incoming S, say S1, S2, ... , Sk where k >= 1B, and you want to check one by one to see if T has its subsequence. In this scenario, how would you change your code?

#### Credits:
Special thanks to [@pbrother](https://leetcode.com/pbrother/) for adding this problem and creating all test cases.

## Solutions (Rust)

### 1. Two Pointers
```Rust
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s = s.chars();
        let mut t = t.chars();
        while let Some(ch_s) = s.next() {
            loop {
                match t.next() {
                    Some(ch_t) => {
                        if ch_t == ch_s {
                            break;
                        }
                    },
                    None => return false,
                };
            }
        }
        true
    }
}
```

### 2. Binary Search
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut char_index = HashMap::new();
        for (i, c) in t.chars().enumerate() {
            char_index.entry(c).or_insert(Vec::new()).push(i);
        }

        let mut prev = 0;
        for c in s.chars() {
            if let Some(v) = char_index.get(&c) {
                match v.binary_search(&prev) {
                    Ok(i) => prev += 1,
                    Err(i) => {
                        if i == v.len() {
                            return false;
                        }
                        prev = v[i] + 1;
                    },
                };
            } else {
                return false;
            }
        }
        true
    }
}
```
