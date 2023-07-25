# 2433. Find The Original Array of Prefix Xor
You are given an **integer** array `pref` of size `n`. Find and return *the array* `arr` *of size* `n` *that satisfies*:

* `pref[i] = arr[0] ^ arr[1] ^ ... ^ arr[i]`.

Note that `^` denotes the **bitwise-xor** operation.

It can be proven that the answer is **unique**.

#### Example 1:
<pre>
<strong>Input:</strong> pref = [5,2,0,3,1]
<strong>Output:</strong> [5,7,2,3,2]
<strong>Explanation:</strong> From the array [5,7,2,3,2] we have the following:
- pref[0] = 5.
- pref[1] = 5 ^ 7 = 2.
- pref[2] = 5 ^ 7 ^ 2 = 0.
- pref[3] = 5 ^ 7 ^ 2 ^ 3 = 3.
- pref[4] = 5 ^ 7 ^ 2 ^ 3 ^ 2 = 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> pref = [13]
<strong>Output:</strong> [13]
<strong>Explanation:</strong> We have pref[0] = arr[0] = 13.
</pre>

#### Constraints:
* <code>1 <= pref.length <= 10<sup>5</sup></code>
* <code>0 <= pref[i] <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut arr = pref;

        for i in (1..arr.len()).rev() {
            arr[i] ^= arr[i - 1];
        }

        arr
    }
}
```
