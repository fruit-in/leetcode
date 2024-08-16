# 1239. Maximum Length of a Concatenated String with Unique Characters
You are given an array of strings `arr`. A string `s` is formed by the **concatenation** of a **subsequence** of `arr` that has **unique characters**.

Return *the **maximum** possible length* of `s`.

A **subsequence** is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.

#### Example 1:
<pre>
<strong>Input:</strong> arr = ["un","iq","ue"]
<strong>Output:</strong> 4
<strong>Explanation:</strong> All the valid concatenations are:
- ""
- "un"
- "iq"
- "ue"
- "uniq" ("un" + "iq")
- "ique" ("iq" + "ue")
Maximum length is 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = ["cha","r","act","ers"]
<strong>Output:</strong> 6
<strong>Explanation:</strong> Possible longest valid concatenations are "chaers" ("cha" + "ers") and "acters" ("act" + "ers").
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = ["abcdefghijklmnopqrstuvwxyz"]
<strong>Output:</strong> 26
<strong>Explanation:</strong> The only string in arr has all 26 characters.
</pre>

#### Constraints:
* `1 <= arr.length <= 16`
* `1 <= arr[i].length <= 26`
* `arr[i]` contains only lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut masks = Vec::new();
        let mut ret = 0;

        for s in &arr {
            let mask = s.bytes().fold(0_i32, |acc, c| acc | (1 << (c - b'a')));

            if mask.count_ones() == s.len() as u32 {
                masks.push(mask);
            }
        }

        for x in 0..2_i32.pow(masks.len() as u32) {
            let mut mask = 0;
            let mut flag = true;

            for i in 0..masks.len() {
                if x & (1 << i) != 0 {
                    if mask ^ masks[i] != mask | masks[i] {
                        flag = false;
                        break;
                    }

                    mask |= masks[i];
                }
            }

            if flag {
                ret = ret.max(mask.count_ones());
            }
        }

        ret as i32
    }
}
```
