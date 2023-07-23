# 150. 逆波兰表达式求值
根据[逆波兰表示法](https://baike.baidu.com/item/%E9%80%86%E6%B3%A2%E5%85%B0%E5%BC%8F/128437)，求表达式的值。

有效的运算符包括```+```,```-```,```*```,```/```。每个运算对象可以是整数，也可以是另一个逆波兰表达式。

#### 说明:
* 整数除法只保留整数部分。
* 给定逆波兰表达式总是有效的。换句话说，表达式总会得出有效数值且不存在除数为 0 的情况。

#### 示例 1:
<pre>
<strong>输入:</strong> ["2", "1", "+", "3", "*"]
<strong>输出:</strong> 9
<strong>解释:</strong> ((2 + 1) * 3) = 9
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> ["4", "13", "5", "/", "+"]
<strong>输出:</strong> 6
<strong>解释:</strong> (4 + (13 / 5)) = 6
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
<strong>输出:</strong> 22
<strong>解释:</strong>
  ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
= ((10 * (6 / (12 * -11))) + 17) + 5
= ((10 * (6 / -132)) + 17) + 5
= ((10 * 0) + 17) + 5
= (0 + 17) + 5
= 17 + 5
= 22
</pre>

## 题解 (Rust)

### 1. 栈
```Rust
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut nums: Vec<i32> = Vec::new();
        for token in tokens {
            if ["+", "-", "*", "/"].contains(&token.as_str()) {
                let temp1 = nums.pop().unwrap();
                let temp2 = nums.pop().unwrap();
                match token.as_str() {
                    "+" => nums.push(temp2 + temp1),
                    "-" => nums.push(temp2 - temp1),
                    "*" => nums.push(temp2 * temp1),
                    "/" => nums.push(temp2 / temp1),
                    _ => (),
                }
            } else {
                nums.push(token.parse().unwrap());
            }
        }
        nums.pop().unwrap()
    }
}
```
