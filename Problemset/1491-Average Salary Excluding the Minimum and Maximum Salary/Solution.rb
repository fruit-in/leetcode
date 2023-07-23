# @param {Integer[]} salary
# @return {Float}
def average(salary)
    min_salary = salary.min
    max_salary = salary.max
    sum_salary = salary.sum

    return 1.0 * (sum_salary - min_salary - max_salary) / (salary.length - 2)
end
