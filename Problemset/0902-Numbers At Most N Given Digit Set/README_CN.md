# 902. 最大为 N 的数字组合
给定一个按 **非递减顺序** 排列的数字数组 `digits` 。你可以用任意次数 `digits[i]` 来写的数字。例如，如果 `digits = ['1','3','5']`，我们可以写数字，如 `'13'`, `'551'`, 和 `'1351315'`。

返回 *可以生成的小于或等于给定整数 `n` 的正整数的个数* 。

#### 示例 1:
<pre>
<strong>输入:</strong> digits = ["1","3","5","7"], n = 100
<strong>输出:</strong> 20
<strong>解释:</strong>
可写出的 20 个数字是：
1, 3, 5, 7, 11, 13, 15, 17, 31, 33, 35, 37, 51, 53, 55, 57, 71, 73, 75, 77.
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> digits = ["1","4","9"], n = 1000000000
<strong>输出:</strong> 29523
<strong>解释:</strong>
我们可以写 3 个一位数字，9 个两位数字，27 个三位数字，
81 个四位数字，243 个五位数字，729 个六位数字，
2187 个七位数字，6561 个八位数字和 19683 个九位数字。
总共，可以使用D中的数字写出 29523 个整数。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> digits = ["7"], n = 8
<strong>输出:</strong> 1
</pre>

#### 提示:
* `1 <= digits.length <= 9`
* `digits[i].length == 1`
* `digits[i]` 是从 `'1'` 到 `'9'` 的数
* `digits` 中的所有值都 **不同**
* `digits` 按 **非递减顺序** 排列
* <code>1 <= n <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let digits = digits
            .iter()
            .map(|d| d.parse::<u8>().unwrap() + b'0')
            .collect::<Vec<_>>();
        let digitsn = n.to_string().into_bytes();
        let q = digits.len();
        let n = digitsn.len();
        let mut ret = n;

        if q > 1 {
            ret = q * (q.pow(n as u32) - 1) / (q - 1);

            for i in 0..n {
                match digits.binary_search(&digitsn[i]) {
                    Ok(j) => ret -= q.pow((n - i - 1) as u32) * (q - j - 1),
                    Err(j) => {
                        ret -= q.pow((n - i - 1) as u32) * (q - j);
                        break;
                    }
                }
            }
        } else if vec![digits[0]; n] > digitsn {
            ret -= 1;
        }

        ret as i32
    }
}
```
