# 640. Solve the Equation
Solve a given equation and return the value of `'x'` in the form of a string `"x=#value"`. The equation contains only `'+'`, `'-'` operation, the variable `'x'` and its coefficient. You should return `"No solution"` if there is no solution for the equation, or `"Infinite solutions"` if there are infinite solutions for the equation.

If there is exactly one solution for the equation, we ensure that the value of `'x'` is an integer.

#### Example 1:
<pre>
<strong>Input:</strong> equation = "x+5-3+x=6+x-2"
<strong>Output:</strong> "x=2"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> equation = "x=x"
<strong>Output:</strong> "Infinite solutions"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> equation = "2x=x"
<strong>Output:</strong> "x=0"
</pre>

#### Constraints:
* `3 <= equation.length <= 1000`
* `equation` has exactly one `'='`.
* `equation` consists of integers with an absolute value in the range `[0, 100]` without any leading zeros, and the variable `'x'`.

## Solutions (Rust)

### 1. Solution
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
