# 1017. Convert to Base -2
Given a number ```N```, return a string consisting of ```"0"```s and ```"1"```s that represents its value in base **```-2```** (negative two).

The returned string must have no leading zeroes, unless the string is ```"0"```.

#### Example 1:
<pre>
<strong>Input:</strong> 2
<strong>Output:</strong> "110"
<strong>Explanation:</strong> (-2) ^ 2 + (-2) ^ 1 = 2
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 3
<strong>Output:</strong> "111"
<strong>Explanation:</strong> (-2) ^ 2 + (-2) ^ 1 + (-2) ^ 0 = 3
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> 4
<strong>Output:</strong> "100"
<strong>Explanation:</strong> (-2) ^ 2 = 4
</pre>

#### Note:
1. ```0 <= N <= 10^9```

## Solutions (Rust)

### 1. Mathematical
```Rust
impl Solution {
    pub fn base_neg2(n: i32) -> String {
        if n == 0 {
            return "0".to_string();
        }

        let mut n = n;
        let mut ret = Vec::new();

        while n != 0 {
            let x = n % 4;
            match x {
                0 => ret.push("00"),
                1 if n != 1 => ret.push("01"),
                2 => ret.push("10"),
                3 => ret.push("11"),
                _ => ret.push("1"),
            }
            n = n / 4 + x / 2;
        }

        ret.reverse();
        ret.concat()
    }
}
```
