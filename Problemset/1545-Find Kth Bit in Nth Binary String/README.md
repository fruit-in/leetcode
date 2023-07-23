# 1545. Find Kth Bit in Nth Binary String
Given two positive integers `n` and `k`, the binary string  <code>S<sub>n</sub></code> is formed as follows:
* <code>S<sub>1</sub> = "0"</code>
* <code>S<sub>i</sub> = S<sub>i-1</sub> + "1" + reverse(invert(S<sub>i-1</sub>))</code> for `i > 1`

Where `+` denotes the concatenation operation, `reverse(x)` returns the reversed string `x`, and `invert(x)` inverts all the bits in `x` (0 changes to 1 and 1 changes to 0).

For example, the first 4 strings in the above sequence are:
* <code>S<sub>1</sub> = "0"</code>
* <code>S<sub>2</sub> = "0<b>1</b>1"</code>
* <code>S<sub>3</sub> = "011<b>1</b>001"</code>
* <code>S<sub>4</sub> = "0111001<b>1</b>0110001"</code>

Return *the* <code>k<sup>th</sup></code> *bit in* <code>S<sub>n</sub></code>. It is guaranteed that `k` is valid for the given `n`.

#### Example 1:
<pre>
<b>Input:</b> n = 3, k = 1
<b>Output:</b> "0"
<b>Explanation:</b> S<sub>3</sub> is "<b><u>0</u></b>111001". The first bit is "0".
</pre>

#### Example 2:
<pre>
<b>Input:</b> n = 4, k = 11
<b>Output:</b> "1"
<b>Explanation:</b> S<sub>4</sub> is "0111001101<b><u>1</u></b>0001". The 11th bit is "1".
</pre>

#### Example 3:
<pre>
<b>Input:</b> n = 1, k = 1
<b>Output:</b> "0"
</pre>

#### Example 4:
<pre>
<b>Input:</b> n = 2, k = 3
<b>Output:</b> "1"
</pre>

#### Constraints:
* `1 <= n <= 20`
* <code>1 <= k <= 2<sup>n</sup> - 1</code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let mut bits = vec![false];

        loop {
            if let Some(&b) = bits.get(k as usize - 1) {
                return char::from(b as u8 + b'0');
            }

            let mut x = bits.clone().iter().map(|&b| !b).rev().collect::<Vec<_>>();
            bits.push(true);
            bits.append(&mut x);
        }
    }
}
```
