# 1432. Max Difference You Can Get From Changing an Integer
You are given an integer `num`. You will apply the following steps exactly **two** times:
* Pick a digit `x (0 <= x <= 9)`.
* Pick another digit `y (0 <= y <= 9)`. The digit `y` can be equal to `x`.
* Replace all the occurrences of `x` in the decimal representation of `num` by `y`.
* The new integer **cannot** have any leading zeros, also the new integer **cannot** be 0.

Let `a` and `b` be the results of applying the operations to `num` the first and second times, respectively.

Return *the max difference* between `a` and `b`.

#### Example 1:
<pre>
<strong>Input:</strong> num = 555
<strong>Output:</strong> 888
<strong>Explanation:</strong> The first time pick x = 5 and y = 9 and store the new integer in a.
The second time pick x = 5 and y = 1 and store the new integer in b.
We have now a = 999 and b = 111 and max difference = 888
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num = 9
<strong>Output:</strong> 8
<strong>Explanation:</strong> The first time pick x = 9 and y = 9 and store the new integer in a.
The second time pick x = 9 and y = 1 and store the new integer in b.
We have now a = 9 and b = 1 and max difference = 8
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> num = 123456
<strong>Output:</strong> 820000
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> num = 10000
<strong>Output:</strong> 80000
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> num = 9288
<strong>Output:</strong> 8700
</pre>

#### Constraints:
* `1 <= num <= 10^8`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        let num = num.to_string();
        let max = match num.chars().find(|&c| c < '9') {
            Some(c) => num.replace(c, "9"),
            None => num.clone(),
        };
        let min = if num.starts_with('1') {
            match num.chars().find(|&c| c > '1') {
                Some(c) => num.replace(c, "0"),
                None => num,
            }
        } else {
            num.replace(num.chars().next().unwrap(), "1")
        };

        max.parse::<i32>().unwrap() - min.parse::<i32>().unwrap()
    }
}
```
