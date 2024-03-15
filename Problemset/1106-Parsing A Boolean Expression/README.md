# 1106. Parsing A Boolean Expression
A **boolean expression** is an expression that evaluates to either `true` or `false`. It can be in one of the following shapes:

* `'t'` that evaluates to `true`.
* `'f'` that evaluates to `false`.
* `'!(subExpr)'` that evaluates to **the logical NOT** of the inner expression `subExpr`.
* `'&(subExpr1, subExpr2, ..., subExprn)'` that evaluates to **the logical AND** of the inner expressions `subExpr1, subExpr2, ..., subExprn` where `n >= 1`.
* `'|(subExpr1, subExpr2, ..., subExprn)'` that evaluates to **the logical OR** of the inner expressions `subExpr1, subExpr2, ..., subExprn` where `n >= 1`.

Given a string `expression` that represents a **boolean expression**, return *the evaluation of that expression*.

It is **guaranteed** that the given expression is valid and follows the given rules.

#### Example 1:
<pre>
<strong>Input:</strong> expression = "&(|(f))"
<strong>Output:</strong> false
<strong>Explanation:</strong>
First, evaluate |(f) --> f. The expression is now "&(f)".
Then, evaluate &(f) --> f. The expression is now "f".
Finally, return false.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> expression = "|(f,f,f,t)"
<strong>Output:</strong> true
<strong>Explanation:</strong> The evaluation of (false OR false OR false OR true) is true.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> expression = "!(&(f,t))"
<strong>Output:</strong> true
<strong>Explanation:</strong>
First, evaluate &(f,t) --> (false AND true) --> false --> f. The expression is now "!(f)".
Then, evaluate !(f) --> NOT false --> true. We return true.
</pre>

#### Constraints:
* <code>1 <= expression.length <= 2 * 10<sup>4</sup></code>
* expression[i] is one following characters: `'('`, `')'`, `'&'`, `'|'`, `'!'`, `'t'`, `'f'`, and `','`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut ops = vec![];
        let mut bools = vec![];

        for ch in expression.chars() {
            match ch {
                '!' | '&' | '|' => ops.push(ch),
                't' | 'f' => bools.push(Some(ch == 't')),
                ')' => match ops.pop() {
                    Some('!') => {
                        let tmp = !bools.pop().unwrap().unwrap();
                        bools.pop();
                        bools.push(Some(tmp));
                    }
                    Some('&') => {
                        let mut tmp = true;
                        while let Some(Some(b)) = bools.pop() {
                            tmp &= b;
                        }
                        bools.push(Some(tmp));
                    }
                    Some('|') => {
                        let mut tmp = false;
                        while let Some(Some(b)) = bools.pop() {
                            tmp |= b;
                        }
                        bools.push(Some(tmp));
                    }
                    _ => {
                        let tmp = bools.pop().unwrap();
                        bools.pop();
                        bools.push(tmp);
                    }
                },
                '(' => bools.push(None),
                _ => (),
            }
        }

        bools[0].unwrap()
    }
}
```
