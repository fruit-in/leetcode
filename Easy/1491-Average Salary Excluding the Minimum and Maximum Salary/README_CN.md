# 1491. 去掉最低工资和最高工资后的工资平均值
给你一个整数数组 `salary` ，数组里每个数都是 **唯一** 的，其中 `salary[i]` 是第 `i` 个员工的工资。

请你返回去掉最低工资和最高工资以后，剩下员工工资的平均值。

#### 示例 1:
<pre>
<strong>输入:</strong> salary = [4000,3000,1000,2000]
<strong>输出:</strong> 2500.00000
<strong>解释:</strong> 最低工资和最高工资分别是 1000 和 4000 。
去掉最低工资和最高工资以后的平均工资是 (2000+3000)/2= 2500
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> salary = [1000,2000,3000]
<strong>输出:</strong> 2000.00000
<strong>解释:</strong> 最低工资和最高工资分别是 1000 和 3000 。
去掉最低工资和最高工资以后的平均工资是 (2000)/1= 2000
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> salary = [6000,5000,4000,3000,2000,1000]
<strong>输出:</strong> 3500.00000
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> salary = [8000,9000,2000,3000,6000,1000]
<strong>输出:</strong> 4750.00000
</pre>

#### 提示:
* `3 <= salary.length <= 100`
* `10^3 <= salary[i] <= 10^6`
* `salary[i]` 是唯一的。
* 与真实值误差在 `10^-5` 以内的结果都将视为正确答案。

## 题解 (Ruby)

### 1. 题解
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

## 题解 (Rust)

### 1. 题解
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
