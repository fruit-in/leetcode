# 319. Bulb Switcher
There are *n* bulbs that are initially off. You first turn on all the bulbs. Then, you turn off every second bulb. On the third round, you toggle every third bulb (turning on if it's off or turning off if it's on). For the *i*-th round, you toggle every *i* bulb. For the *n*-th round, you only toggle the last bulb. Find how many bulbs are on after *n* rounds.

#### Example:
<pre>
<strong>Input:</strong> 3
<strong>Output:</strong> 1
<strong>Explanation:</strong>
At first, the three bulbs are <strong>[off, off, off]</strong>.
After first round, the three bulbs are <strong>[on, on, on]</strong>.
After second round, the three bulbs are <strong>[on, off, on]</strong>.
After third round, the three bulbs are <strong>[on, off, off]</strong>.

So you should return 1, because there is only one bulb is on.
</pre>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64).sqrt() as i32
    }
}
```
