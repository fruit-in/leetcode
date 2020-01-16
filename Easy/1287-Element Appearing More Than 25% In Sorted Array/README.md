# 1287. Element Appearing More Than 25% In Sorted Array
Given an integer array **sorted** in non-decreasing order, there is exactly one integer in the array that occurs more than 25% of the time.

Return that integer.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [1,2,2,6,6,6,6,7,10]
<strong>Output:</strong> 6
</pre>

#### Constraints:
* ```1 <= arr.length <= 10^4```
* ```0 <= arr[i] <= 10^5```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut l = 0;

        while let Ok(r) = arr.binary_search(&arr[l]) {
            if r - l >= arr.len() / 4 {
                return arr[l];
            }

            l = r + 1;
        }

        -1
    }
}
```
