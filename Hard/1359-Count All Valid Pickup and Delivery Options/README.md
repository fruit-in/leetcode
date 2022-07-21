# 1359. Count All Valid Pickup and Delivery Options
Given `n` orders, each order consist in pickup and delivery services.

Count all valid pickup/delivery possible sequences such that delivery(i) is always after of pickup(i).

Since the answer may be too large, return it modulo 10^9 + 7.

#### Example 1:
<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> Unique order (P1, D1), Delivery 1 always is after of Pickup 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> 6
<strong>Explanation:</strong> All possible orders:
(P1,P2,D1,D2), (P1,P2,D2,D1), (P1,D1,P2,D2), (P2,P1,D1,D2), (P2,P1,D2,D1) and (P2,D2,P1,D1).
This is an invalid order (P1,D2,P2,D1) because Pickup 2 is after of Delivery 2.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 3
<strong>Output:</strong> 90
</pre>

#### Constraints:
* `1 <= n <= 500`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        let mut ret = 1_i64;

        for i in 1..n as i64 {
            ret = (ret * (2 * i + 1) * (i + 1)) % 1_000_000_007;
        }

        ret as i32
    }
}
```
