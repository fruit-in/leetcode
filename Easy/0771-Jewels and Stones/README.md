# 771. Jewels and Stones
You're given strings <code>J</code> representing the types of stones that are jewels, and <code>S</code> representing the stones you have.  Each character in <code>S</code> is a type of stone you have.  You want to know how many of the stones you have are also jewels.

The letters in <code>J</code> are guaranteed distinct, and all characters in <code>J</code> and <code>S</code> are letters. Letters are case sensitive, so <code>"a"</code> is considered a different type of stone from <code>"A"</code>.

#### Example 1:
<pre>
<strong>Input:</strong> J = "aA", S = "aAAbbbb"
<strong>Output:</strong> 3
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> J = "z", S = "ZZ"
<strong>Output:</strong> 0
</pre>

#### Note:
* <code>S</code> and <code>J</code> will consist of letters and have length at most 50.
* The characters in <code>J</code> are distinct.

## Solutions (Rust)

### 1. HashMap
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut jewels = HashMap::new();
        for ch_j in j.chars() {
            jewels.insert(ch_j, 0);
        }
        for ch_s in s.chars() {
            if let Some(i) = jewels.get(&ch_s) {
                jewels.insert(ch_s, i + 1);
            }
        }
        jewels.values().sum()
    }
}
```
