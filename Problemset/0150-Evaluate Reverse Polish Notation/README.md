# 150. Evaluate Reverse Polish Notation
Evaluate the value of an arithmetic expression in [Reverse Polish Notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation).

Valid operators are <code>+</code>, <code>-</code>, <code>*</code>, <code>/</code>. Each operand may be an integer or another expression.

#### Note:
* Division between two integers should truncate toward zero.
* The given RPN expression is always valid. That means the expression would always evaluate to a result and there won't be any divide by zero operation.

#### Example 1:
<pre>
<strong>Input:</strong> ["2", "1", "+", "3", "*"]
<strong>Output:</strong> 9
<strong>Explanation:</strong> ((2 + 1) * 3) = 9
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> ["4", "13", "5", "/", "+"]
<strong>Output:</strong> 6
<strong>Explanation:</strong> (4 + (13 / 5)) = 6
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
<strong>Output:</strong> 22
<strong>Explanation:</strong>
  ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
= ((10 * (6 / (12 * -11))) + 17) + 5
= ((10 * (6 / -132)) + 17) + 5
= ((10 * 0) + 17) + 5
= (0 + 17) + 5
= 17 + 5
= 22
</pre>

## Solutions (Rust)

### 1. Stack
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
