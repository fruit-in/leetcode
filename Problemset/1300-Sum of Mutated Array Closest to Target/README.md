# 1300. Sum of Mutated Array Closest to Target
Given an integer array `arr` and a target value `target`, return the integer `value` such that when we change all the integers larger than `value` in the given array to be equal to `value`, the sum of the array gets as close as possible (in absolute difference) to `target`.

In case of a tie, return the minimum such integer.

Notice that the answer is not neccesarilly a number from `arr`.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [4,9,3], target = 10
<strong>Output:</strong> 3
<strong>Explanation:</strong> When using 3 arr converts to [3, 3, 3] which sums 9 and that's the optimal answer.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [2,3,5], target = 10
<strong>Output:</strong> 5
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [60864,25176,27249,21296,20204], target = 56803
<strong>Output:</strong> 11361
</pre>

#### Constraints:
* <code>1 <= arr.length <= 10<sup>4</sup></code>
* <code>1 <= arr[i], target <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        let mut arr = arr;
        let mut prefix_sum = 0;
        let mut diff = target;
        let mut value = 0;

        arr.push(0);
        arr.sort_unstable();

        for i in 1..arr.len() {
            let mut l = arr[i - 1];
            let mut r = arr[i];

            while l <= r {
                let m = (l + r) / 2;
                let sum = prefix_sum + m * (arr.len() - i) as i32;

                if sum == target {
                    return m;
                } else if sum < target {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            }

            let mut rdiff = (prefix_sum + r * (arr.len() - i) as i32 - target).abs();
            let mut ldiff = (prefix_sum + l * (arr.len() - i) as i32 - target).abs();

            if r >= arr[i - 1] && rdiff < diff {
                diff = rdiff;
                value = r;
            }
            if l <= arr[i] && ldiff < diff {
                diff = ldiff;
                value = l;
            }

            prefix_sum += arr[i];
        }

        value
    }
}
```
