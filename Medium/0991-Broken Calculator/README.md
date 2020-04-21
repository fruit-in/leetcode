# 991. Broken Calculator
On a broken calculator that has a number showing on its display, we can perform two operations:
* **Double**: Multiply the number on the display by 2, or;
* **Decrement**: Subtract 1 from the number on the display.

Initially, the calculator is displaying the number ```X```.

Return the minimum number of operations needed to display the number ```Y```.

#### Example 1:
<pre>
<strong>Input:</strong> X = 2, Y = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> Use double operation and then decrement operation {2 -> 4 -> 3}.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> X = 5, Y = 8
<strong>Output:</strong> 2
<strong>Explanation:</strong> Use decrement and then double {5 -> 4 -> 8}.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> X = 3, Y = 10
<strong>Output:</strong> 3
<strong>Explanation:</strong> Use double, decrement and double {3 -> 6 -> 5 -> 10}.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> X = 1024, Y = 1
<strong>Output:</strong> 1023
<strong>Explanation:</strong> Use decrement operations 1023 times.
</pre>

#### Note:
1. ```1 <= X <= 10^9```
2. ```1 <= Y <= 10^9```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn broken_calc(x: i32, y: i32) -> i32 {
        let mut y = y;
        let mut ret = 0;

        while x < y {
            ret += 1;

            if y % 2 == 1 {
                y += 1;
            } else {
                y /= 2;
            }
        }

        ret + x - y
    }
}
```
