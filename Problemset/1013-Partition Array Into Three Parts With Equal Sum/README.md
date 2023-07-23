# 1013. Partition Array Into Three Parts With Equal Sum
Given an array ```A``` of integers, return ```true``` if and only if we can partition the array into three **non-empty** parts with equal sums.

Formally, we can partition the array if we can find indexes ```i+1 < j``` with ```(A[0] + A[1] + ... + A[i] == A[i+1] + A[i+2] + ... + A[j-1] == A[j] + A[j-1] + ... + A[A.length - 1])```

#### Example 1:
<pre>
<strong>Input:</strong> [0,2,1,-6,6,-7,9,1,2,0,1]
<strong>Output:</strong> true
<strong>Explanation:</strong> 0 + 2 + 1 = -6 + 6 - 7 + 9 + 1 = 2 + 0 + 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [0,2,1,-6,6,7,9,-1,2,0,1]
<strong>Output:</strong> false
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> [3,3,6,5,-2,2,5,1,-9,4]
<strong>Output:</strong> true
<strong>Explanation:</strong> 3 + 3 = 6 = 5 - 2 + 2 + 5 + 1 - 9 + 4
</pre>

#### Note:
1. ```3 <= A.length <= 50000```
2. ```-10000 <= A[i] <= 10000```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
        let mut total_sum: i32 = a.iter().sum();
        if total_sum % 3 != 0 {
            return false;
        }

        let mut i = 0;
        let mut part_sum = 0;
        while i < a.len() {
            part_sum += a[i];
            if part_sum == total_sum / 3 {
                break;
            }
            i += 1;
        }

        let mut j = a.len() - 1;
        part_sum = 0;
        while j > 0 {
            part_sum += a[j];
            if part_sum == total_sum / 3 {
                break;
            }
            j -= 1;
        }

        i + 1 < j
    }
}
```
