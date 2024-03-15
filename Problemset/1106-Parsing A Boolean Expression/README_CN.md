# 1106. 解析布尔表达式
**布尔表达式** 是计算结果不是 `true` 就是 `false` 的表达式。有效的表达式需遵循以下约定：

* `'t'`，运算结果为 `true`
* `'f'`，运算结果为 `false`
* `'!(subExpr)'`，运算过程为对内部表达式 `subExpr` 进行 **逻辑非**（NOT）运算
* `'&(subExpr1, subExpr2, ..., subExprn)'`，运算过程为对 2 个或以上内部表达式 `subExpr1, subExpr2, ..., subExprn` 进行 **逻辑与**（AND）运算
* `'|(subExpr1, subExpr2, ..., subExprn)'`，运算过程为对 2 个或以上内部表达式 `subExpr1, subExpr2, ..., subExprn` 进行 **逻辑或**（OR）运算

给你一个以字符串形式表述的 [布尔表达式](https://baike.baidu.com/item/%E5%B8%83%E5%B0%94%E8%A1%A8%E8%BE%BE%E5%BC%8F/1574380?fr=aladdin) `expression`，返回该式的运算结果。

题目测试用例所给出的表达式均为有效的布尔表达式，遵循上述约定。

#### 示例 1:
<pre>
<strong>输入:</strong> expression = "&(|(f))"
<strong>输出:</strong> false
<strong>解释:</strong>
首先，计算 |(f) --> f ，表达式变为 "&(f)" 。
接着，计算 &(f) --> f ，表达式变为 "f" 。
最后，返回 false 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> expression = "|(f,f,f,t)"
<strong>输出:</strong> true
<strong>解释:</strong> 计算 (false OR false OR false OR true) ，结果为 true 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> expression = "!(&(f,t))"
<strong>输出:</strong> true
<strong>解释:</strong>
首先，计算 &(f,t) --> (false AND true) --> false --> f ，表达式变为 "!(f)" 。
接着，计算 !(f) --> NOT false --> true ，返回 true 。
</pre>

#### 提示:
* <code>1 <= expression.length <= 2 * 10<sup>4</sup></code>
* `expression[i]` 为 `'('`、`')'`、`'&'`、`'|'`、`'!'`、`'t'`、`'f'` 和 `','` 之一

## 题解 (Rust)

### 1. 题解
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
