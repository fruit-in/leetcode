# 1716. Calculate Money in Leetcode Bank
Hercy wants to save money for his first car. He puts money in the Leetcode bank **every day**.

He starts by putting in `$1` on Monday, the first day. Every day from Tuesday to Sunday, he will put in `$1` more than the day before. On every subsequent Monday, he will put in `$1` more than the **previous Monday**.

Given `n`, return *the total amount of money he will have in the Leetcode bank at the end of the* <code>n<sup>th</sup></code> *day*.

#### Example 1:
<pre>
<strong>Input:</strong> n = 4
<strong>Output:</strong> 10
<strong>Explanation:</strong> After the 4th day, the total is 1 + 2 + 3 + 4 = 10.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 10
<strong>Output:</strong> 37
<strong>Explanation:</strong> After the 10th day, the total is (1 + 2 + 3 + 4 + 5 + 6 + 7) + (2 + 3 + 4) = 37. Notice that on the 2nd Monday, Hercy only puts in $2.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 20
<strong>Output:</strong> 96
<strong>Explanation:</strong> After the 20th day, the total is (1 + 2 + 3 + 4 + 5 + 6 + 7) + (2 + 3 + 4 + 5 + 6 + 7 + 8) + (3 + 4 + 5 + 6 + 7 + 8) = 96.
</pre>

#### Constraints:
* `1 <= n <= 1000`

## Solutions (Ruby)

### 1. Mathematical
```Ruby
# @param {Integer} n
# @return {Integer}
def total_money(n)
  x = n / 7
  y = n % 7

  28 * x + x * (x - 1) * 7 / 2 + (x + 1) * y + y * (y - 1) / 2
end
```

## Solutions (Rust)

### 1. Mathematical
```Rust
impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let x = n / 7;
        let y = n % 7;

        28 * x + x * (x - 1) * 7 / 2 + (x + 1) * y + y * (y - 1) / 2
    }
}
```
