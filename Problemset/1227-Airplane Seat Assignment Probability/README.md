# 1227. Airplane Seat Assignment Probability
`n` passengers board an airplane with exactly `n` seats. The first passenger has lost the ticket and picks a seat randomly. But after that, the rest of the passengers will:

* Take their own seat if it is still available, and
* Pick other seats randomly when they find their seat occupied

Return *the probability that the* <code>n<sup>th</sup></code> *person gets his own seat*.

#### Example 1:
<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 1.00000
<strong>Explanation:</strong> The first person can only get the first seat.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> 0.50000
<strong>Explanation:</strong> The second person has a probability of 0.5 to get the second seat (when first person gets the first seat).
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn nth_person_gets_nth_seat(n: i32) -> f64 {
        match n {
            1 => 1.0,
            _ => 0.5,
        }
    }
}
```
