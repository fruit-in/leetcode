# 592. 分数加减运算
给定一个表示分数加减运算表达式的字符串，你需要返回一个字符串形式的计算结果。 这个结果应该是不可约分的分数，即[最简分数](https://baike.baidu.com/item/%E6%9C%80%E7%AE%80%E5%88%86%E6%95%B0)。 如果最终结果是一个整数，例如 ```2```，你需要将它转换成分数形式，其分母为 ```1```。所以在上述例子中, ```2``` 应该被转换为 ```2/1```。

#### 示例 1:
<pre>
<strong>输入:</strong> "-1/2+1/2"
<strong>输出:</strong> "0/1"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "-1/2+1/2+1/3"
<strong>输出:</strong> "1/3"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> "1/3-1/2"
<strong>输出:</strong> "-1/6"
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> "5/3+1/3"
<strong>输出:</strong> "2/1"
</pre>

#### 说明:
1. 输入和输出字符串只包含 ```'0'``` 到 ```'9'``` 的数字，以及 ```'/'```, ```'+'``` 和 ```'-'```。 
2. 输入和输出分数格式均为 ```±分子/分母```。如果输入的第一个分数或者输出的分数是正数，则 ```'+'``` 会被省略掉。
3. 输入只包含合法的**最简分数**，每个分数的**分子**与**分母**的范围是  [1,10]。 如果分母是1，意味着这个分数实际上是一个整数。
4. 输入的分数个数范围是 [1,10]。
5. **最终结果**的分子与分母保证是 32 位整数范围内的有效整数。

## 题解 (Rust)

### 1. 最小公倍数
```Rust
impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let mut numerator = 0;
        let mut denominator = 1;
        let mut sum = 0;
        let mut is_numerator = true;
        let mut sign = 1;

        for ch in expression.bytes() {
            match ch {
                b'+' => {
                    sum += sign * numerator * (2520 / denominator);
                    numerator = 0;
                    is_numerator = true;
                    sign = 1;
                },
                b'-' => {
                    sum += sign * numerator * (2520 / denominator);
                    numerator = 0;
                    is_numerator = true;
                    sign = -1;
                },
                b'/' => {
                    is_numerator = false;
                    denominator = 0;
                },
                n if is_numerator => {
                    numerator *= 10;
                    numerator += (n - b'0') as i32;
                },
                n => {
                    denominator *= 10;
                    denominator += (n - b'0') as i32;
                }
            }
        }

        sum += sign * numerator * (2520 / denominator);
        denominator = 2520;

        for i in &[2, 2, 2, 3, 3, 5, 7] {
            if sum % i == 0 {
                sum /= i;
                denominator /= i;
            }
        }

        sum.to_string() + "/" + &denominator.to_string()
    }
}
```
