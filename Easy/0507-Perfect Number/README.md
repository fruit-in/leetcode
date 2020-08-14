# 507. Perfect Number
We define the Perfect Number is a **positive** integer that is equal to the sum of all its **positive** divisors except itself.

Now, given an **integer** n, write a function that returns true when it is a perfect number and false when it is not.

#### Example:
<pre>
<strong>Input:</strong> 28
<strong>Output:</strong> True
<strong>Explanation:</strong> 28 = 1 + 2 + 4 + 7 + 14
</pre>

**Note:** The input number **n** will not exceed 100,000,000. (1e8)

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer} num
# @return {Boolean}
def check_perfect_number(num)
    return false if num <= 1

    sum = 1
    i = 2

    while i * i < num
        sum += i + num / i if num % i == 0
        i += 1
    end

    sum += i if i * i == num

    return sum == num
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num <= 1 {
            return false;
        }

        let mut sum = 1;
        let mut i = 2;

        while i * i < num {
            if num % i == 0 {
                sum += i + num / i;
            }
            i += 1;
        }

        if i * i == num {
            sum += i;
        }

        sum == num
    }
}
```
