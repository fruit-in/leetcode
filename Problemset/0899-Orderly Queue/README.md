# 899. Orderly Queue
You are given a string `s` and an integer `k`. You can choose one of the first `k` letters of `s` and append it at the end of the string.

Return *the lexicographically smallest string you could have after applying the mentioned step any number of moves*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "cba", k = 1
<strong>Output:</strong> "acb"
<strong>Explanation:</strong>
In the first move, we move the 1st character 'c' to the end, obtaining the string "bac".
In the second move, we move the 1st character 'b' to the end, obtaining the final result "acb".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "baaca", k = 3
<strong>Output:</strong> "aaabc"
<strong>Explanation:</strong>
In the first move, we move the 1st character 'b' to the end, obtaining the string "aacab".
In the second move, we move the 3rd character 'c' to the end, obtaining the final result "aaabc".
</pre>

#### Constraints:
* `1 <= k <= s.length <= 1000`
* `s` consist of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        let mut s = s.into_bytes();
        let n = s.len();
        let mut i = 0;

        if k > 1 {
            s.sort_unstable();
        } else {
            for j in 1..n {
                for k in 0..n {
                    if s[(i + k) % n] < s[(j + k) % n] {
                        break;
                    } else if s[(i + k) % n] > s[(j + k) % n] {
                        i = j;
                        break;
                    }
                }
            }

            s.rotate_left(i);
        }

        String::from_utf8(s).unwrap()
    }
}
```
