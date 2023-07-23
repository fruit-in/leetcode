# 1529. Bulb Switcher IV
There is a room with `n` bulbs, numbered from `0` to `n-1`, arranged in a row from left to right. Initially all the bulbs are **turned off**.

Your task is to obtain the configuration represented by `target` where `target[i]` is '1' if the i-th bulb is turned on and is '0' if it is turned off.

You have a switch to flip the state of the bulb, a flip operation is defined as follows:
* Choose **any** bulb (index `i`) of your current configuration.
* Flip each bulb from index `i` to `n-1`.

When any bulb is flipped it means that if it is 0 it changes to 1 and if it is 1 it changes to 0.

Return the **minimum** number of flips required to form `target`.

#### Example 1:
<pre>
<strong>Input:</strong> target = "10111"
<strong>Output:</strong> 3
<strong>Explanation:</strong> Initial configuration "00000".
flip from the third bulb:  "00000" -> "00111"
flip from the first bulb:  "00111" -> "11000"
flip from the second bulb:  "11000" -> "10111"
We need at least 3 flip operations to form target.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> target = "101"
<strong>Output:</strong> 3
<strong>Explanation:</strong> "000" -> "111" -> "100" -> "101".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> target = "00000"
<strong>Output:</strong> 0
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> target = "001011101"
<strong>Output:</strong> 5
</pre>

#### Constraints:
* `1 <= target.length <= 10^5`
* `target[i] == '0'` or `target[i] == '1'`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_flips(target: String) -> i32 {
        target
            .bytes()
            .fold(0, |ret, ch| ret + ((ret % 2) as u8 ^ (ch - b'0')) as i32)
    }
}
```
