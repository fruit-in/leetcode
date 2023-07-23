# 788. 旋转数字
我们称一个数 X 为好数, 如果它的每位数字逐个地被旋转 180 度后，我们仍可以得到一个有效的，且和 X 不同的数。要求每位数字都要被旋转。

如果一个数的每位数字被旋转以后仍然还是一个数字， 则这个数是有效的。0, 1, 和 8 被旋转后仍然是它们自己；2 和 5 可以互相旋转成对方；6 和 9 同理，除了这些以外其他的数字旋转以后都不再是有效的数字。

现在我们有一个正整数 ```N```, 计算从 ```1``` 到 ```N``` 中有多少个数 X 是好数？

<pre>
<strong>示例:</strong>
<strong>输入:</strong> 10
<strong>输出:</strong> 4
<strong>解释:</strong>
在[1, 10]中有四个好数： 2, 5, 6, 9。
注意 1 和 10 不是好数, 因为他们在旋转之后不变。
</pre>

#### 注意:
* N 的取值范围是 ```[1, 10000]```。

## 题解 (Rust)

### 1. 暴力法
```Rust
impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        (2..=n).map(|num| num.to_string())
               .filter(|s| "2569".chars().any(|c| s.contains(c)))
               .filter(|s| s.chars().all(|c| "0182569".contains(c)))
               .count() as i32
    }
}
```

### 2. 数学
```Rust
impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        fn helper(n: i32, allow_same: bool) -> i32 {
            let x = (n as f64).log10() as u32;
            let a = n / 10_i32.pow(x);
            let b = n % 10_i32.pow(x);

            match (a, allow_same) {
                (1, true) => 7_i32.pow(x) + helper(b, true),
                (2, true) => 2 * 7_i32.pow(x) + helper(b, true),
                (3, true)|(4, true) => 3 * 7_i32.pow(x),
                (5, true) => 3 * 7_i32.pow(x) + helper(b, true),
                (6, true) => 4 * 7_i32.pow(x) + helper(b, true),
                (7, true) => 5 * 7_i32.pow(x),
                (8, true) => 5 * 7_i32.pow(x) + helper(b, true),
                (9, true) => 6 * 7_i32.pow(x) + helper(b, true),
                (_, true) => 1,

                (1, false) => 7_i32.pow(x) - 3_i32.pow(x) + helper(b, false),
                (2, false) => 2 * 7_i32.pow(x) - 2 * 3_i32.pow(x) + helper(b, true),
                (3, false)|(4, false) => 3 * 7_i32.pow(x) - 2 * 3_i32.pow(x),
                (5, false) => 3 * 7_i32.pow(x) - 2 * 3_i32.pow(x) + helper(b, true),
                (6, false) => 4 * 7_i32.pow(x) - 2 * 3_i32.pow(x) + helper(b, true),
                (7, false) => 5 * 7_i32.pow(x) - 2 * 3_i32.pow(x),
                (8, false) => 5 * 7_i32.pow(x) - 2 * 3_i32.pow(x) + helper(b, false),
                (9, false) => 6 * 7_i32.pow(x) - 3 * 3_i32.pow(x) + helper(b, true),
                (_, false) => 0,
            }
        }

        helper(n, false)
    }
}
```
