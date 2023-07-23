# 2399. Check Distances Between Same Letters
You are given a **0-indexed** string `s` consisting of only lowercase English letters, where each letter in `s` appears **exactly twice**. You are also given a **0-indexed** integer array `distance` of length `26`.

Each letter in the alphabet is numbered from `0` to `25` (i.e. `'a' -> 0`, `'b' -> 1`, `'c' -> 2`, ... , `'z' -> 25`).

In a **well-spaced** string, the number of letters between the two occurrences of the <code>i<sup>th</sup></code> letter is `distance[i]`. If the <code>i<sup>th</sup></code> letter does not appear in `s`, then `distance[i]` can be **ignored**.

Return `true` *if* `s` *is a **well-spaced** string, otherwise return* `false`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "abaccb", distance = [1,3,0,5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
<strong>Output:</strong> true
<strong>Explanation:</strong>
- 'a' appears at indices 0 and 2 so it satisfies distance[0] = 1.
- 'b' appears at indices 1 and 5 so it satisfies distance[1] = 3.
- 'c' appears at indices 3 and 4 so it satisfies distance[2] = 0.
Note that distance[3] = 5, but since 'd' does not appear in s, it can be ignored.
Return true because s is a well-spaced string.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "aa", distance = [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
<strong>Output:</strong> false
<strong>Explanation:</strong>
- 'a' appears at indices 0 and 1 so there are zero letters between them.
Because distance[0] = 1, s is not a well-spaced string.
</pre>

#### Constraints:
* `2 <= s.length <= 52`
* `s` consists only of lowercase English letters.
* Each letter appears in `s` exactly twice.
* `distance.length == 26`
* `0 <= distance[i] <= 50`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut distance_s = vec![-1; 26];

        for (i, c) in s.bytes().enumerate() {
            if distance_s[(c - b'a') as usize] == -1 {
                distance_s[(c - b'a') as usize] = i as i32 + 1;
            } else {
                distance_s[(c - b'a') as usize] = i as i32 - distance_s[(c - b'a') as usize];
            }
        }

        distance_s
            .iter()
            .zip(distance.iter())
            .all(|(a, b)| a == b || *a == -1)
    }
}
```
