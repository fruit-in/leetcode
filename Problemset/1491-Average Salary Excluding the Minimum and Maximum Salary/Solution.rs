impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let min_salary = salary.iter().min().unwrap();
        let max_salary = salary.iter().max().unwrap();
        let sum_salary = salary.iter().sum::<i32>();

        (sum_salary - min_salary - max_salary) as f64 / (salary.len() - 2) as f64
    }
}
