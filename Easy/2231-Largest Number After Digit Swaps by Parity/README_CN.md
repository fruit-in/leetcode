# 2231. 按奇偶性交换后的最大数字
给你一个正整数 `num` 。你可以交换 `num` 中 **奇偶性** 相同的任意两位数字（即，都是奇数或者偶数）。

返回交换 **任意** 次之后 `num` 的 **最大** 可能值。

#### 示例 1:
<pre>
<strong>输入:</strong> num = 1234
<strong>输出:</strong> 3412
<strong>解释:</strong> 交换数字 3 和数字 1 ，结果得到 3214 。
交换数字 2 和数字 4 ，结果得到 3412 。
注意，可能存在其他交换序列，但是可以证明 3412 是最大可能值。
注意，不能交换数字 4 和数字 1 ，因为它们奇偶性不同。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num = 65875
<strong>输出:</strong> 87655
<strong>解释:</strong> 交换数字 8 和数字 6 ，结果得到 85675 。
交换数字 5 和数字 7 ，结果得到 87655 。
注意，可能存在其他交换序列，但是可以证明 87655 是最大可能值。
</pre>

#### 提示:
* <code>1 <= num <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn largest_integer(num: i32) -> i32 {
        let mut num = num;
        let mut odd_digits = vec![];
        let mut even_digits = vec![];
        let mut is_odds = vec![];
        let mut ret = 0;

        while num > 0 {
            if num % 2 == 1 {
                odd_digits.push(num % 10);
                is_odds.push(true);
            } else {
                even_digits.push(num % 10);
                is_odds.push(false);
            }
            num /= 10;
        }

        odd_digits.sort_unstable();
        even_digits.sort_unstable();

        while let Some(is_odd) = is_odds.pop() {
            if is_odd {
                ret = ret * 10 + odd_digits.pop().unwrap();
            } else {
                ret = ret * 10 + even_digits.pop().unwrap();
            }
        }

        ret
    }
}
```
