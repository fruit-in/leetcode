# 640. 求解方程
求解一个给定的方程，将`x`以字符串 `"x=#value"` 的形式返回。该方程仅包含 `'+'` ， `'-'` 操作，变量 `x` 和其对应系数。

如果方程没有解或存在的解不为整数，请返回 `"No solution"` 。如果方程有无限解，则返回 `“Infinite solutions”` 。

题目保证，如果方程中只有一个解，则 `'x'` 的值是一个整数。

#### 示例 1:
<pre>
<strong>输入:</strong> equation = "x+5-3+x=6+x-2"
<strong>输出:</strong> "x=2"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> equation = "x=x"
<strong>输出:</strong> "Infinite solutions"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> equation = "2x=x"
<strong>输出:</strong> "x=0"
</pre>

#### 提示:
* `3 <= equation.length <= 1000`
* `equation` 只有一个 `'='`.
* 方程由绝对值在 `[0, 100]`  范围内且无任何前导零的整数和变量 `'x'` 组成。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn solve_equation(equation: String) -> String {
        let equation = equation.replace("-", "+-");
        let (left, right) = equation.split_once('=').unwrap();
        let mut coefficient = 0;
        let mut num = 0;

        for s in left.split('+') {
            if s.ends_with('x') {
                if s == "-x" {
                    coefficient -= 1;
                } else {
                    coefficient += s.trim_end_matches('x').parse::<i32>().unwrap_or(1);
                }
            } else if !s.is_empty() {
                num += s.parse::<i32>().unwrap();
            }
        }
        for s in right.split('+') {
            if s.ends_with('x') {
                if s == "-x" {
                    coefficient += 1;
                } else {
                    coefficient -= s.trim_end_matches('x').parse::<i32>().unwrap_or(1);
                }
            } else if !s.is_empty() {
                num -= s.parse::<i32>().unwrap();
            }
        }

        match (coefficient, num) {
            (0, 0) => "Infinite solutions".to_string(),
            (0, _) => "No solution".to_string(),
            _ => format!("x={}", -num / coefficient),
        }
    }
}
```
