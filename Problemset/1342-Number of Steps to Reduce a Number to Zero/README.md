# 1342. Number of Steps to Reduce a Number to Zero
Given a non-negative integer ```num```, return the number of steps to reduce it to zero. If the current number is even, you have to divide it by 2, otherwise, you have to subtract 1 from it.

#### Example 1:
<pre>
<strong>Input:</strong> num = 14
<strong>Output:</strong> 6
<strong>Explanation:</strong>
Step 1) 14 is even; divide by 2 and obtain 7.
Step 2) 7 is odd; subtract 1 and obtain 6.
Step 3) 6 is even; divide by 2 and obtain 3.
Step 4) 3 is odd; subtract 1 and obtain 2.
Step 5) 2 is even; divide by 2 and obtain 1.
Step 6) 1 is odd; subtract 1 and obtain 0.
</pre>
  
#### Example 2:
<pre>
<strong>Input:</strong> num = 8
<strong>Output:</strong> 4
<strong>Explanation:</strong>
Step 1) 8 is even; divide by 2 and obtain 4.
Step 2) 4 is even; divide by 2 and obtain 2.
Step 3) 2 is even; divide by 2 and obtain 1.
Step 4) 1 is odd; subtract 1 and obtain 0.
</pre>
  
#### Example 3:
<pre>
<strong>Input:</strong> num = 123
<strong>Output:</strong> 12
</pre>

#### Constraints:
* ```0 <= num <= 10^6```

## Solutions (Ruby)

### 1. Simulation
```Ruby
# @param {Integer} num
# @return {Integer}
def number_of_steps (num)
    ret = 0

    while num != 0
        if num % 2 == 0
            num /= 2
        else
            num -= 1
        end
        ret += 1
    end

    return ret
end
```

## Solutions (Rust)

### 1. Simulation
```Rust
impl Solution {
    pub fn number_of_steps (num: i32) -> i32 {
        let mut num = num;
        let mut ret = 0;

        while num != 0 {
            match num % 2 {
                0 => num /= 2,
                _ => num -= 1,
            }
            ret += 1;
        }

        ret
    }
}
```
