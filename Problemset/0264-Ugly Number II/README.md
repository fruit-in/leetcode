# 264. Ugly Number II
Write a program to find the ```n```-th ugly number.

Ugly numbers are **positive numbers** whose prime factors only include ```2, 3, 5```.

#### Example:
<pre>
<strong>Input:</strong> n = 10
<strong>Output:</strong> 12
<strong>Explanation:</strong> 1, 2, 3, 4, 5, 6, 8, 9, 10, 12 is the sequence of the first 10 ugly numbers.
</pre>

#### Note:
1. ```1``` is typically treated as an ugly number.
2. ```n``` **does not exceed 1690**.

## Solutions (Rust)

### 1. Dynamic Programming
```Rust
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut uglys = vec![1];
        let mut p2 = 0;
        let mut p3 = 0;
        let mut p5 = 0;

        for _ in 1..n {
            let min_ugly = (2 * uglys[p2]).min(3 * uglys[p3]).min(5 * uglys[p5]);
            uglys.push(min_ugly);

            if min_ugly == 2 * uglys[p2] {
                p2 += 1;
            }
            if min_ugly == 3 * uglys[p3] {
                p3 += 1;
            }
            if min_ugly == 5 * uglys[p5] {
                p5 += 1;
            }
        }

        uglys[n as usize - 1]
    }
}
```
