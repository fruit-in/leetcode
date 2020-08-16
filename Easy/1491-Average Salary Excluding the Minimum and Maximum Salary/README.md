# 1491. Average Salary Excluding the Minimum and Maximum Salary
Given an array of **unique** integers `salary` where `salary[i]` is the salary of the employee `i`.

Return the average salary of employees excluding the minimum and maximum salary.

#### Example 1:
<pre>
<strong>Input:</strong> salary = [4000,3000,1000,2000]
<strong>Output:</strong> 2500.00000
<strong>Explanation:</strong> Minimum salary and maximum salary are 1000 and 4000 respectively.
Average salary excluding minimum and maximum salary is (2000+3000)/2= 2500
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> salary = [1000,2000,3000]
<strong>Output:</strong> 2000.00000
<strong>Explanation:</strong> Minimum salary and maximum salary are 1000 and 3000 respectively.
Average salary excluding minimum and maximum salary is (2000)/1= 2000
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> salary = [6000,5000,4000,3000,2000,1000]
<strong>Output:</strong> 3500.00000
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> salary = [8000,9000,2000,3000,6000,1000]
<strong>Output:</strong> 4750.00000
</pre>

#### Constraints:
* `3 <= salary.length <= 100`
* `10^3 <= salary[i] <= 10^6`
* `salary[i]` is unique.
* Answers within `10^-5` of the actual value will be accepted as correct.

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[]} salary
# @return {Float}
def average(salary)
    min_salary = salary.min
    max_salary = salary.max
    sum_salary = salary.sum

    return 1.0 * (sum_salary - min_salary - max_salary) / (salary.length - 2)
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let min_salary = salary.iter().min().unwrap();
        let max_salary = salary.iter().max().unwrap();
        let sum_salary = salary.iter().sum::<i32>();

        (sum_salary - min_salary - max_salary) as f64 / (salary.len() - 2) as f64
    }
}
```
