# 66. 加一
给定一个由**整数**组成的**非空**数组所表示的非负整数，在该数的基础上加一。

最高位数字存放在数组的首位， 数组中每个元素只存储**单个**数字。

你可以假设除了整数 0 之外，这个整数不会以零开头。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,2,3]
<strong>输出:</strong> [1,2,4]
<strong>解释:</strong> 输入数组表示数字 123。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [4,3,2,1]
<strong>输出:</strong> [4,3,2,2]
<strong>解释:</strong> 输入数组表示数字 4321。
</pre>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut i = digits.len() - 1;
        loop {
            if digits[i] != 9 {
                digits[i] += 1;
                return digits;
            } else {
                digits[i] = 0;
                if i > 0 {
                    i -= 1;
                } else {
                    digits.insert(0, 1);
                    return digits;
                }
            }
        }
    }
}
```
