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
