# 1023. Camelcase Matching
Given an array of strings `queries` and a string `pattern`, return a boolean array `answer` where `answer[i]` is `true` if `queries[i]` matches `pattern`, and `false` otherwise.

A query word `queries[i]` matches `pattern` if you can insert lowercase English letters pattern so that it equals the query. You may insert each character at any position and you may not insert any characters.

#### Example 1:
<pre>
<strong>Input:</strong> queries = ["FooBar","FooBarTest","FootBall","FrameBuffer","ForceFeedBack"], pattern = "FB"
<strong>Output:</strong> [true,false,true,true,false]
<strong>Explanation:</strong> "FooBar" can be generated like this "F" + "oo" + "B" + "ar".
"FootBall" can be generated like this "F" + "oot" + "B" + "all".
"FrameBuffer" can be generated like this "F" + "rame" + "B" + "uffer".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> queries = ["FooBar","FooBarTest","FootBall","FrameBuffer","ForceFeedBack"], pattern = "FoBa"
<strong>Output:</strong> [true,false,true,false,false]
<strong>Explanation:</strong> "FooBar" can be generated like this "Fo" + "o" + "Ba" + "r".
"FootBall" can be generated like this "Fo" + "ot" + "Ba" + "ll".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> queries = ["FooBar","FooBarTest","FootBall","FrameBuffer","ForceFeedBack"], pattern = "FoBaT"
<strong>Output:</strong> [false,true,false,false,false]
<strong>Explanation:</strong> "FooBarTest" can be generated like this "Fo" + "o" + "Ba" + "r" + "T" + "est".
</pre>

#### Constraints:
* `1 <= pattern.length, queries.length <= 100`
* `1 <= queries[i].length <= 100`
* `queries[i]` and `pattern` consist of English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let pattern = pattern.as_bytes();
        let mut ret = vec![true; queries.len()];

        for i in 0..queries.len() {
            let query = queries[i].as_bytes();
            let mut j = 0;

            for k in 0..query.len() {
                if j < pattern.len() && query[k] == pattern[j] {
                    j += 1;
                } else if query[k].is_ascii_uppercase() {
                    ret[i] = false;
                    break;
                }
            }

            ret[i] &= j == pattern.len();
        }

        ret
    }
}
```
