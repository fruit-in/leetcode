# 2515. Shortest Distance to Target String in a Circular Array
You are given a **0-indexed circular** string array `words` and a string `target`. A **circular array** means that the array's end connects to the array's beginning.

* Formally, the next element of `words[i]` is `words[(i + 1) % n]` and the previous element of `words[i]` is `words[(i - 1 + n) % n]`, where `n` is the length of `words`.

Starting from `startIndex`, you can move to either the next word or the previous word with `1` step at a time.

Return *the **shortest** distance needed to reach the string* `target`. If the string `target` does not exist in `words`, return `-1`.

#### Example 1:
<pre>
<strong>Input:</strong> words = ["hello","i","am","leetcode","hello"], target = "hello", startIndex = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> We start from index 1 and can reach "hello" by
- moving 3 units to the right to reach index 4.
- moving 2 units to the left to reach index 4.
- moving 4 units to the right to reach index 0.
- moving 1 unit to the left to reach index 0.
The shortest distance to reach "hello" is 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> words = ["a","b","leetcode"], target = "leetcode", startIndex = 0
<strong>Output:</strong> 1
<strong>Explanation:</strong> We start from index 0 and can reach "leetcode" by
- moving 2 units to the right to reach index 3.
- moving 1 unit to the left to reach index 3.
The shortest distance to reach "leetcode" is 1.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> words = ["i","eat","leetcode"], target = "ate", startIndex = 0
<strong>Output:</strong> -1
<strong>Explanation:</strong> Since "ate" does not exist in words, we return -1.
</pre>

#### Constraints:
* `1 <= words.length <= 100`
* `1 <= words[i].length <= 100`
* `words[i]` and `target` consist of only lowercase English letters.
* `0 <= startIndex < words.length`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn closet_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let start_index = start_index as usize;
        let n = words.len();

        for i in 0..n {
            if words[(start_index + i) % n] == target || words[(start_index + n - i) % n] == target
            {
                return i as i32;
            }
        }

        -1
    }
}
```
